import Foundation

public enum ConversationHelpers {
    private static let maxItemsPerThread = 500
    private static let maxItemText = 20_000
    private static let toolOutputRecentItems = 40
    private static let noTruncateToolTypes: Set<String> = ["fileChange", "commandExecution"]

    public static func mergeStreamingText(existing: String, delta: String) -> String {
        guard !delta.isEmpty else { return existing }
        if existing.isEmpty { return delta }
        if delta == existing { return existing }
        if delta.hasPrefix(existing) { return delta }
        if existing.hasPrefix(delta) { return existing }
        let maxOverlap = min(existing.count, delta.count)
        for length in stride(from: maxOverlap, to: 0, by: -1) {
            let prefix = String(delta.prefix(length))
            if existing.hasSuffix(prefix) {
                return existing + delta.dropFirst(length)
            }
        }
        return existing + delta
    }

    public static func normalizeItem(_ item: ConversationItem) -> ConversationItem {
        var next = item
        switch item.kind {
        case .message:
            next.text = truncateText(item.text ?? "")
        case .reasoning:
            next.summary = truncateText(item.summary ?? "")
            next.content = truncateText(item.content ?? "")
        case .diff:
            next.diff = truncateText(item.diff ?? "")
        case .tool:
            let isNoTruncate = noTruncateToolTypes.contains(item.toolType ?? "")
            next.title = truncateText(item.title ?? "", maxLength: 200)
            next.detail = truncateText(item.detail ?? "", maxLength: 2000)
            if !isNoTruncate, let output = item.output {
                next.output = truncateText(output)
            }
            if let changes = item.changes, !isNoTruncate {
                next.changes = changes.map { change in
                    var mutable = change
                    if let diff = change.diff {
                        mutable.diff = truncateText(diff)
                    }
                    return mutable
                }
            }
        default:
            break
        }
        return next
    }

    public static func prepareThreadItems(_ items: [ConversationItem]) -> [ConversationItem] {
        var filtered: [ConversationItem] = []
        for item in items {
            if let last = filtered.last,
               item.kind == .message,
               item.role == .assistant,
               last.kind == .review,
               last.state == .completed,
               item.text?.trimmingCharacters(in: .whitespacesAndNewlines) ==
                last.text?.trimmingCharacters(in: .whitespacesAndNewlines) {
                continue
            }
            filtered.append(item)
        }

        let normalized = filtered.map { normalizeItem($0) }
        let limited = normalized.count > maxItemsPerThread ? Array(normalized.suffix(maxItemsPerThread)) : normalized
        let cutoff = max(0, limited.count - toolOutputRecentItems)
        return limited.enumerated().map { index, item in
            guard index < cutoff, item.kind == .tool, let output = item.output else { return item }
            var trimmed = item
            trimmed.output = truncateText(output)
            if let changes = item.changes {
                trimmed.changes = changes.map { change in
                    var mutable = change
                    if let diff = change.diff {
                        mutable.diff = truncateText(diff)
                    }
                    return mutable
                }
            }
            return trimmed
        }
    }

    public static func upsertItem(_ list: [ConversationItem], item: ConversationItem) -> [ConversationItem] {
        if let index = list.firstIndex(where: { $0.id == item.id }) {
            var next = list
            var merged = next[index]
            merged = mergeItems(base: merged, incoming: item)
            next[index] = merged
            return next
        }
        return list + [item]
    }

    public static func buildConversationItem(from raw: JSONValue) -> ConversationItem? {
        guard case .object(let item) = raw else { return nil }
        let type = asString(item["type"])
        let id = asString(item["id"])
        guard !type.isEmpty, !id.isEmpty else { return nil }

        if type == "agentMessage" {
            let text = asString(item["text"])
            return ConversationItem(id: id, kind: .message, role: .assistant, text: text.isEmpty ? "[message]" : text)
        }
        if type == "userMessage" {
            let content = item["content"]?.arrayValue ?? []
            let text = userInputsToText(content)
            return ConversationItem(id: id, kind: .message, role: .user, text: text.isEmpty ? "[message]" : text)
        }
        if type == "reasoning" {
            let summary = joinStringArray(item["summary"])
            let content = joinStringArray(item["content"])
            return ConversationItem(id: id, kind: .reasoning, summary: summary, content: content)
        }
        if type == "commandExecution" {
            let command = joinStringArray(item["command"])
            let durationMs = asNumber(item["durationMs"] ?? item["duration_ms"])
            return ConversationItem(
                id: id,
                kind: .tool,
                title: command.isEmpty ? "Command" : "Command: \(command)",
                detail: asString(item["cwd"]),
                status: asString(item["status"]),
                output: asString(item["aggregatedOutput"]),
                durationMs: durationMs
            ).withToolType(type)
        }
        if type == "fileChange" {
            let changes = item["changes"]?.arrayValue ?? []
            let normalizedChanges: [ToolChange] = changes.compactMap { changeValue in
                guard case .object(let change) = changeValue else { return nil }
                let path = asString(change["path"])
                guard !path.isEmpty else { return nil }
                let kindValue = change["kind"]
                let kindType = kindValue?.stringValue ?? kindValue?.objectValue?["type"]?.stringValue ?? ""
                let diff = asString(change["diff"])
                return ToolChange(path: path, kind: kindType.isEmpty ? nil : kindType.lowercased(), diff: diff.isEmpty ? nil : diff)
            }
            let formatted = normalizedChanges.compactMap { change -> String? in
                let prefix: String
                switch change.kind {
                case "add": prefix = "A"
                case "delete": prefix = "D"
                case "modify", "update": prefix = "M"
                default: prefix = ""
                }
                return [prefix, change.path].filter { !$0.isEmpty }.joined(separator: " ")
            }
            let paths = formatted.joined(separator: ", ")
            let diffOutput = normalizedChanges.compactMap { $0.diff }.joined(separator: "\n\n")
            return ConversationItem(
                id: id,
                kind: .tool,
                title: "File changes",
                detail: paths.isEmpty ? "Pending changes" : paths,
                status: asString(item["status"]),
                output: diffOutput,
                changes: normalizedChanges
            ).withToolType(type)
        }
        if type == "mcpToolCall" {
            let server = asString(item["server"])
            let tool = asString(item["tool"])
            let args = item["arguments"].map { prettyJSON($0) } ?? ""
            return ConversationItem(
                id: id,
                kind: .tool,
                title: "Tool: \(server)\(tool.isEmpty ? "" : " / \(tool)")",
                detail: args,
                status: asString(item["status"]),
                output: asString(item["result"] ?? item["error"])
            ).withToolType(type)
        }
        if type == "collabToolCall" || type == "collabAgentToolCall" {
            let tool = asString(item["tool"])
            let status = asString(item["status"])
            let sender = asString(item["senderThreadId"] ?? item["sender_thread_id"])
            let receivers = normalizeStringList(item["receiverThreadId"] ?? item["receiver_thread_id"])
                + normalizeStringList(item["receiverThreadIds"] ?? item["receiver_thread_ids"])
                + normalizeStringList(item["newThreadId"] ?? item["new_thread_id"])
            let prompt = asString(item["prompt"])
            let agentsState = formatCollabAgentStates(item["agentStatus"] ?? item["agentsStates"] ?? item["agents_states"])
            let detailParts = [sender.isEmpty ? nil : "From \(sender)"]
                .compactMap { $0 }
                + (receivers.isEmpty ? [] : ["→ \(receivers.joined(separator: ", "))"])
            let outputParts = [prompt, agentsState].filter { !$0.isEmpty }
            return ConversationItem(
                id: id,
                kind: .tool,
                title: tool.isEmpty ? "Collab tool call" : "Collab: \(tool)",
                detail: detailParts.joined(separator: " "),
                status: status,
                output: outputParts.joined(separator: "\n\n")
            ).withToolType("collabToolCall")
        }
        if type == "webSearch" {
            return ConversationItem(
                id: id,
                kind: .tool,
                title: "Web search",
                detail: asString(item["query"]),
                status: nil,
                output: nil
            ).withToolType(type)
        }
        if type == "imageView" {
            return ConversationItem(
                id: id,
                kind: .tool,
                title: "Image view",
                detail: asString(item["path"]),
                status: nil,
                output: nil
            ).withToolType(type)
        }
        if type == "enteredReviewMode" || type == "exitedReviewMode" {
            return ConversationItem(
                id: id,
                kind: .review,
                text: asString(item["review"]),
                state: type == "enteredReviewMode" ? .started : .completed
            )
        }
        return nil
    }

    public static func buildConversationItemFromThreadItem(_ raw: JSONValue) -> ConversationItem? {
        guard case .object(let item) = raw else { return nil }
        let type = asString(item["type"])
        let id = asString(item["id"])
        guard !type.isEmpty, !id.isEmpty else { return nil }

        if type == "userMessage" {
            let content = item["content"]?.arrayValue ?? []
            let text = userInputsToText(content)
            return ConversationItem(id: id, kind: .message, role: .user, text: text.isEmpty ? "[message]" : text)
        }
        if type == "agentMessage" {
            return ConversationItem(id: id, kind: .message, role: .assistant, text: asString(item["text"]))
        }
        if type == "reasoning" {
            let summary = joinStringArray(item["summary"])
            let content = joinStringArray(item["content"])
            return ConversationItem(id: id, kind: .reasoning, summary: summary, content: content)
        }
        return buildConversationItem(from: raw)
    }

    // MARK: - Helpers

    private static func truncateText(_ text: String, maxLength: Int = maxItemText) -> String {
        guard text.count > maxLength else { return text }
        let index = text.index(text.startIndex, offsetBy: max(0, maxLength - 3))
        return String(text[..<index]) + "..."
    }

    private static func mergeItems(base: ConversationItem, incoming: ConversationItem) -> ConversationItem {
        var merged = base
        if let role = incoming.role { merged.role = role }
        if let text = incoming.text { merged.text = text }
        if let summary = incoming.summary { merged.summary = summary }
        if let content = incoming.content { merged.content = content }
        if let title = incoming.title { merged.title = title }
        if let diff = incoming.diff { merged.diff = diff }
        if let state = incoming.state { merged.state = state }
        if let toolType = incoming.toolType { merged.toolType = toolType }
        if let detail = incoming.detail { merged.detail = detail }
        if let status = incoming.status { merged.status = status }
        if let output = incoming.output { merged.output = output }
        if let durationMs = incoming.durationMs { merged.durationMs = durationMs }
        if let changes = incoming.changes { merged.changes = changes }
        return merged
    }

    private static func asString(_ value: JSONValue?) -> String {
        value?.asString() ?? ""
    }

    private static func asNumber(_ value: JSONValue?) -> Double? {
        value?.asNumber()
    }

    private static func joinStringArray(_ value: JSONValue?) -> String {
        if let array = value?.arrayValue {
            return array.map { $0.asString() }.joined(separator: "\n")
        }
        return asString(value)
    }

    private static func normalizeStringList(_ value: JSONValue?) -> [String] {
        if let array = value?.arrayValue {
            return array.map { $0.asString() }.filter { !$0.isEmpty }
        }
        let single = asString(value)
        return single.isEmpty ? [] : [single]
    }

    private static func formatCollabAgentStates(_ value: JSONValue?) -> String {
        guard case .object(let dict)? = value else { return "" }
        let entries = dict.compactMap { key, stateValue -> String? in
            let status = stateValue.objectValue?["status"]?.asString() ?? stateValue.asString()
            if status.isEmpty { return key }
            return "\(key): \(status)"
        }
        return entries.joined(separator: "\n")
    }

    private static func userInputsToText(_ inputs: [JSONValue]) -> String {
        let parts = inputs.compactMap { inputValue -> String? in
            guard case .object(let input) = inputValue else { return nil }
            let type = asString(input["type"])
            switch type {
            case "text":
                return asString(input["text"])
            case "skill":
                let name = asString(input["name"])
                return name.isEmpty ? nil : "$\(name)"
            case "image", "localImage":
                return "[image]"
            default:
                return nil
            }
        }
        return parts.joined(separator: " ").trimmingCharacters(in: .whitespacesAndNewlines)
    }

    private static func prettyJSON(_ value: JSONValue?) -> String {
        guard let value else { return "" }
        guard let data = try? JSONEncoder().encode(value) else { return value.asString() }
        if let object = try? JSONSerialization.jsonObject(with: data),
           let pretty = try? JSONSerialization.data(withJSONObject: object, options: [.prettyPrinted]),
           let string = String(data: pretty, encoding: .utf8) {
            return string
        }
        return value.asString()
    }
}

private extension ConversationItem {
    func withToolType(_ toolType: String) -> ConversationItem {
        var copy = self
        copy.toolType = toolType
        return copy
    }
}
