import SwiftUI
import CodexMonitorModels

struct MemoryView: View {
    @EnvironmentObject private var store: CodexStore
    @State private var query = ""
    @State private var results: [MemorySearchResult] = []
    @State private var status: MemoryStatus?
    @State private var isLoading = false

    @State private var newContent = ""
    @State private var newTags = ""
    @State private var memoryType: MemoryType = .daily

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 16) {
                statusSection
                searchSection
                resultsSection
                composeSection
            }
            .padding()
        }
        .background {
            GradientBackground()
        }
        .navigationTitle("Memory")
        .toolbar {
            ToolbarItem(placement: .topBarTrailing) {
                Button(action: { Task { await refreshAll() } }) {
                    Image(systemName: "arrow.clockwise")
                }
            }
        }
        .task {
            await refreshAll()
        }
        .refreshable {
            await refreshAll()
        }
    }

    private var statusSection: some View {
        VStack(alignment: .leading, spacing: 8) {
            GlassSectionHeader(title: "Status", icon: "chart.bar")
            GlassCard {
                if let status {
                    VStack(alignment: .leading, spacing: 8) {
                        HStack {
                            Text(status.enabled ? "Enabled" : "Disabled")
                                .font(.headline)
                            Spacer()
                            GlassBadge(text: status.embeddingsEnabled ? "Embeddings" : "Text Only")
                        }
                        HStack(spacing: 12) {
                            statusChip(label: "Total", value: status.total, tint: .blue)
                            statusChip(label: "Ready", value: status.ready, tint: .green)
                            statusChip(label: "Pending", value: status.pending, tint: .orange)
                            statusChip(label: "Error", value: status.error, tint: .red)
                        }
                    }
                } else {
                    Text("Memory status unavailable")
                        .foregroundStyle(.secondary)
                }
            }
        }
    }

    private var searchSection: some View {
        VStack(alignment: .leading, spacing: 8) {
            GlassSectionHeader(title: "Search", icon: "magnifyingglass")
            GlassCard {
                HStack(spacing: 12) {
                    TextField("Search memory", text: $query)
                        .textInputAutocapitalization(.never)
                        .autocorrectionDisabled()
                        .submitLabel(.search)
                        .onSubmit { Task { await runSearch() } }

                    GlassIconButton(icon: "magnifyingglass") {
                        Task { await runSearch() }
                    }
                }
            }
        }
    }

    private var resultsSection: some View {
        VStack(alignment: .leading, spacing: 8) {
            GlassSectionHeader(title: "Results", icon: "brain.head.profile")

            if isLoading {
                ProgressView()
                    .frame(maxWidth: .infinity)
            } else if results.isEmpty {
                GlassCard {
                    Text("No memory entries yet.")
                        .foregroundStyle(.secondary)
                }
            } else {
                ForEach(results) { entry in
                    GlassCard {
                        VStack(alignment: .leading, spacing: 8) {
                            Text(entry.content)
                                .font(.body)

                            HStack(spacing: 8) {
                                GlassBadge(text: entry.memoryType.capitalized)
                                if let meta = scoreLabel(for: entry) {
                                    GlassBadge(text: meta)
                                }
                                Spacer()
                                Text(formatDate(entry.createdAt))
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }

                            if !entry.tags.isEmpty {
                                ScrollView(.horizontal, showsIndicators: false) {
                                    HStack(spacing: 8) {
                                        ForEach(entry.tags, id: \.self) { tag in
                                            GlassChip(text: tag, tint: .purple)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    private var composeSection: some View {
        VStack(alignment: .leading, spacing: 8) {
            GlassSectionHeader(title: "Add Memory", icon: "plus")
            GlassCard {
                VStack(alignment: .leading, spacing: 12) {
                    Picker("Type", selection: $memoryType) {
                        Text("Daily").tag(MemoryType.daily)
                        Text("Curated").tag(MemoryType.curated)
                    }
                    .pickerStyle(.segmented)

                    TextEditor(text: $newContent)
                        .frame(minHeight: 120)
                        .overlay(RoundedRectangle(cornerRadius: 12).stroke(Color.white.opacity(0.1)))

                    TextField("Tags (comma separated)", text: $newTags)
                        .textInputAutocapitalization(.never)
                        .autocorrectionDisabled()

                    HStack {
                        Spacer()
                        Button("Save") {
                            Task { await saveMemory() }
                        }
                        .buttonStyle(.borderedProminent)
                        .disabled(newContent.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
                    }
                }
            }
        }
    }

    private func statusChip(label: String, value: Int, tint: Color) -> some View {
        VStack(alignment: .leading, spacing: 2) {
            Text(label)
                .font(.caption2)
                .foregroundStyle(.secondary)
            Text("\(value)")
                .font(.headline)
        }
        .padding(.horizontal, 10)
        .padding(.vertical, 6)
        .background(.ultraThinMaterial, in: RoundedRectangle(cornerRadius: 10))
        .overlay(
            RoundedRectangle(cornerRadius: 10)
                .strokeBorder(tint.opacity(0.4), lineWidth: 1)
        )
    }

    private func runSearch() async {
        let trimmed = query.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else {
            await loadBootstrap()
            return
        }
        isLoading = true
        results = await store.memorySearch(query: trimmed, limit: 20)
        isLoading = false
    }

    private func loadBootstrap() async {
        isLoading = true
        results = await store.memoryBootstrap()
        isLoading = false
    }

    private func refreshAll() async {
        status = await store.memoryStatus()
        await loadBootstrap()
    }

    private func saveMemory() async {
        let tags = newTags
            .split(separator: ",")
            .map { $0.trimmingCharacters(in: .whitespacesAndNewlines) }
            .filter { !$0.isEmpty }
        let workspaceId = store.activeWorkspaceId
        let inserted = await store.memoryAppend(
            type: memoryType,
            content: newContent,
            tags: tags,
            workspaceId: workspaceId
        )
        guard inserted != nil else { return }
        newContent = ""
        newTags = ""
        await refreshAll()
    }

    private func formatDate(_ value: String) -> String {
        if let date = ISO8601DateFormatter().date(from: value) {
            return date.formatted(date: .abbreviated, time: .shortened)
        }
        return value
    }

    private func scoreLabel(for entry: MemorySearchResult) -> String? {
        if let score = entry.score {
            return String(format: "Score %.2f", score)
        }
        if let rank = entry.rank {
            return String(format: "Rank %.2f", rank)
        }
        return nil
    }
}
