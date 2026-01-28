import Foundation

// MARK: - Workspace Models

public struct WorkspaceSettings: Codable, Hashable, Sendable {
    public var sidebarCollapsed: Bool
    public var sortOrder: Int?
    public var groupId: String?
    public var gitRoot: String?
    public var domainId: String?
    public var applyDomainInstructions: Bool?

    public init(
        sidebarCollapsed: Bool = false,
        sortOrder: Int? = nil,
        groupId: String? = nil,
        gitRoot: String? = nil,
        domainId: String? = nil,
        applyDomainInstructions: Bool? = nil
    ) {
        self.sidebarCollapsed = sidebarCollapsed
        self.sortOrder = sortOrder
        self.groupId = groupId
        self.gitRoot = gitRoot
        self.domainId = domainId
        self.applyDomainInstructions = applyDomainInstructions
    }

    enum CodingKeys: String, CodingKey {
        case sidebarCollapsed
        case sortOrder
        case groupId
        case gitRoot
        case domainId
        case applyDomainInstructions
    }
}

public struct WorkspaceGroup: Codable, Hashable, Sendable, Identifiable {
    public var id: String
    public var name: String
    public var sortOrder: Int?
    public var copiesFolder: String?
}

public enum WorkspaceKind: String, Codable, Sendable {
    case main
    case worktree
}

public struct WorktreeInfo: Codable, Hashable, Sendable {
    public var branch: String
}

public struct WorkspaceInfo: Codable, Hashable, Sendable, Identifiable {
    public var id: String
    public var name: String
    public var path: String
    public var connected: Bool
    public var codexBin: String?
    public var kind: WorkspaceKind?
    public var parentId: String?
    public var worktree: WorktreeInfo?
    public var settings: WorkspaceSettings

    enum CodingKeys: String, CodingKey {
        case id, name, path, connected, kind, worktree, settings
        case codexBin = "codex_bin"
        case parentId = "parentId"
    }
}

// MARK: - Settings

public enum AccessMode: String, Codable, Sendable {
    case readOnly = "read-only"
    case current = "current"
    case fullAccess = "full-access"
}

public enum BackendMode: String, Codable, Sendable {
    case local
    case remote
}

public enum ThemePreference: String, Codable, Sendable {
    case system
    case light
    case dark
}

public enum ComposerEditorPreset: String, Codable, Sendable {
    case `default`
    case helpful
    case smart
}

public struct AppSettings: Codable, Hashable, Sendable {
    public var codexBin: String?
    public var backendMode: BackendMode
    public var remoteBackendHost: String
    public var remoteBackendToken: String?
    public var defaultAccessMode: String
    public var composerModelShortcut: String?
    public var composerAccessShortcut: String?
    public var composerReasoningShortcut: String?
    public var composerPlanModeShortcut: String?
    public var newAgentShortcut: String?
    public var newWorktreeAgentShortcut: String?
    public var newCloneAgentShortcut: String?
    public var toggleProjectsSidebarShortcut: String?
    public var toggleGitSidebarShortcut: String?
    public var toggleMemoryPanelShortcut: String?
    public var toggleDebugPanelShortcut: String?
    public var toggleTerminalShortcut: String?
    public var cycleAgentNextShortcut: String?
    public var cycleAgentPrevShortcut: String?
    public var cycleWorkspaceNextShortcut: String?
    public var cycleWorkspacePrevShortcut: String?
    public var lastComposerModelId: String?
    public var lastComposerReasoningEffort: String?
    public var uiScale: Double
    public var theme: ThemePreference
    public var uiFontFamily: String
    public var codeFontFamily: String
    public var codeFontSize: Int
    public var notificationSoundsEnabled: Bool
    public var experimentalCollabEnabled: Bool
    public var experimentalSteerEnabled: Bool
    public var experimentalUnifiedExecEnabled: Bool
    public var dictationEnabled: Bool
    public var dictationModelId: String
    public var dictationPreferredLanguage: String?
    public var dictationHoldKey: String
    public var memory_enabled: Bool?
    public var supabase_url: String?
    public var supabase_anon_key: String?
    public var minimax_api_key: String?
    public var memory_embedding_enabled: Bool?
    public var autoMemory: AutoMemorySettings
    public var composerEditorPreset: ComposerEditorPreset
    public var composerFenceExpandOnSpace: Bool
    public var composerFenceExpandOnEnter: Bool
    public var composerFenceLanguageTags: Bool
    public var composerFenceWrapSelection: Bool
    public var composerFenceAutoWrapPasteMultiline: Bool
    public var composerFenceAutoWrapPasteCodeLike: Bool
    public var composerListContinuation: Bool
    public var composerCodeBlockCopyUseModifier: Bool
    public var workspaceGroups: [WorkspaceGroup]
}

// MARK: - Domains

public struct DomainTheme: Codable, Hashable, Sendable {
    public var icon: String
    public var color: String
    public var accent: String
    public var background: String?
}

public struct Domain: Codable, Hashable, Sendable, Identifiable {
    public var id: String
    public var name: String
    public var description: String?
    public var systemPrompt: String
    public var viewType: String
    public var theme: DomainTheme
    public var defaultModel: String?
    public var defaultAccessMode: String?
    public var defaultReasoningEffort: String?
    public var defaultApprovalPolicy: String?

    public init(
        id: String = UUID().uuidString,
        name: String,
        description: String? = nil,
        systemPrompt: String = "",
        viewType: String = "dashboard",
        theme: DomainTheme,
        defaultModel: String? = nil,
        defaultAccessMode: String? = nil,
        defaultReasoningEffort: String? = nil,
        defaultApprovalPolicy: String? = nil
    ) {
        self.id = id
        self.name = name
        self.description = description
        self.systemPrompt = systemPrompt
        self.viewType = viewType
        self.theme = theme
        self.defaultModel = defaultModel
        self.defaultAccessMode = defaultAccessMode
        self.defaultReasoningEffort = defaultReasoningEffort
        self.defaultApprovalPolicy = defaultApprovalPolicy
    }
}

public struct TrendCard: Codable, Hashable, Sendable, Identifiable {
    public var id: String
    public var label: String
    public var value: String
    public var subLabel: String?
}

public struct TrendListItem: Codable, Hashable, Sendable {
    public var label: String
    public var value: String
    public var subLabel: String?
}

public struct TrendList: Codable, Hashable, Sendable, Identifiable {
    public var id: String
    public var title: String
    public var items: [TrendListItem]
}

public struct TrendSeries: Codable, Hashable, Sendable, Identifiable {
    public var id: String
    public var label: String
    public var points: [Double]
    public var labels: [String]?
}

public struct DomainTrendSnapshot: Codable, Hashable, Sendable {
    public var domainId: String
    public var range: String
    public var updatedAt: String
    public var cards: [TrendCard]
    public var lists: [TrendList]
    public var series: [TrendSeries]?
}

public struct AutoMemorySettings: Codable, Hashable, Sendable {
    public var enabled: Bool
    public var reserveTokensFloor: Int
    public var softThresholdTokens: Int
    public var minIntervalSeconds: Int
    public var maxTurns: Int
    public var maxSnapshotChars: Int
    public var includeToolOutput: Bool
    public var includeGitStatus: Bool
    public var writeDaily: Bool
    public var writeCurated: Bool
}

// MARK: - Memory

public struct MemoryStatus: Codable, Sendable {
    public let enabled: Bool
    public let embeddingsEnabled: Bool
    public let total: Int
    public let pending: Int
    public let ready: Int
    public let error: Int

    enum CodingKeys: String, CodingKey {
        case enabled
        case embeddingsEnabled = "embeddings_enabled"
        case total, pending, ready, error
    }
}

public struct MemorySearchResult: Codable, Sendable, Identifiable {
    public let id: String
    public let content: String
    public let memoryType: String
    public let tags: [String]
    public let workspaceId: String?
    public let createdAt: String
    public let distance: Double?
    public let score: Double?
    public let rank: Float?

    enum CodingKeys: String, CodingKey {
        case id, content, tags
        case memoryType = "memory_type"
        case workspaceId = "workspace_id"
        case createdAt = "created_at"
        case distance, score, rank
    }
}

public struct MemoryEntry: Codable, Sendable {
    public let id: String?
    public let content: String
    public let memoryType: String
    public let tags: [String]
    public let workspaceId: String?
    public let embeddingStatus: String?
    public let createdAt: String?

    enum CodingKeys: String, CodingKey {
        case id, content, tags
        case memoryType = "memory_type"
        case workspaceId = "workspace_id"
        case embeddingStatus = "embedding_status"
        case createdAt = "created_at"
    }
}

public enum MemoryType: String, Codable, Sendable, CaseIterable {
    case daily
    case curated
}

// MARK: - Browser

public struct BrowserSessionCreated: Codable, Sendable {
    public let sessionId: String
}

public struct BrowserSessionList: Codable, Sendable {
    public let sessions: [String]
}

public struct BrowserScreenshot: Codable, Sendable {
    public let base64Png: String
    public let url: String?
    public let title: String?
    public let width: Int?
    public let height: Int?
}

public struct BrowserElement: Codable, Sendable {
    public let tag: String
    public let text: String
    public let name: String?
    public let id: String?
    public let href: String?
}

public struct BrowserSnapshot: Codable, Sendable {
    public let base64Png: String
    public let url: String?
    public let title: String?
    public let width: Int?
    public let height: Int?
    public let elements: [BrowserElement]
}

// MARK: - Skills

public struct SkillValidationResult: Codable, Sendable, Identifiable {
    public var id: String { name }
    public let name: String
    public let path: String
    public let issues: [String]
    public let description: String?
}

public struct SkillOption: Codable, Sendable, Identifiable, Hashable {
    public var id: String { "\(name)|\(path)" }
    public let name: String
    public let path: String
    public let description: String?

    public init(name: String, path: String, description: String? = nil) {
        self.name = name
        self.path = path
        self.description = description
    }
}

// MARK: - App Server Events

public struct AppServerEvent: Codable, Sendable {
    public let workspaceId: String
    public let message: JSONValue

    enum CodingKeys: String, CodingKey {
        case workspaceId = "workspace_id"
        case message
    }
}

public struct TerminalOutputEvent: Codable, Sendable {
    public let workspaceId: String
    public let terminalId: String
    public let data: String
}

// MARK: - Conversation / Threads

public enum MessageRole: String, Codable, Sendable {
    case user
    case assistant
}

public enum ConversationItemKind: String, Codable, Sendable {
    case message
    case reasoning
    case diff
    case review
    case tool
}

public enum ReviewState: String, Codable, Sendable {
    case started
    case completed
}

public struct ToolChange: Codable, Hashable, Sendable {
    public var path: String
    public var kind: String?
    public var diff: String?
}

public struct ConversationItem: Identifiable, Hashable, Sendable {
    public var id: String
    public var kind: ConversationItemKind
    public var role: MessageRole? = nil
    public var text: String? = nil
    public var summary: String? = nil
    public var content: String? = nil
    public var title: String? = nil
    public var diff: String? = nil
    public var state: ReviewState? = nil
    public var toolType: String? = nil
    public var detail: String? = nil
    public var status: String? = nil
    public var output: String? = nil
    public var durationMs: Double? = nil
    public var changes: [ToolChange]? = nil

    public init(
        id: String,
        kind: ConversationItemKind,
        role: MessageRole? = nil,
        text: String? = nil,
        summary: String? = nil,
        content: String? = nil,
        title: String? = nil,
        diff: String? = nil,
        state: ReviewState? = nil,
        toolType: String? = nil,
        detail: String? = nil,
        status: String? = nil,
        output: String? = nil,
        durationMs: Double? = nil,
        changes: [ToolChange]? = nil
    ) {
        self.id = id
        self.kind = kind
        self.role = role
        self.text = text
        self.summary = summary
        self.content = content
        self.title = title
        self.diff = diff
        self.state = state
        self.toolType = toolType
        self.detail = detail
        self.status = status
        self.output = output
        self.durationMs = durationMs
        self.changes = changes
    }
}

public struct ThreadSummary: Codable, Hashable, Sendable, Identifiable {
    public var id: String
    public var name: String
    public var updatedAt: Double

    public init(id: String, name: String, updatedAt: Double) {
        self.id = id
        self.name = name
        self.updatedAt = updatedAt
    }
}

public struct ThreadTurn: Codable, Hashable, Sendable {
    public var id: String?
    public var items: [JSONValue]?
    public var error: JSONValue?

    public init(id: String? = nil, items: [JSONValue]? = nil, error: JSONValue? = nil) {
        self.id = id
        self.items = items
        self.error = error
    }
}

public struct ThreadRecord: Codable, Hashable, Sendable, Identifiable {
    public var id: String
    public var name: String?
    public var title: String?
    public var preview: String?
    public var cwd: String?
    public var createdAt: Double?
    public var updatedAt: Double?
    public var created_at: Double?
    public var updated_at: Double?
    public var turns: [ThreadTurn]?
}

public struct ThreadListResponse: Codable, Sendable {
    public var data: [ThreadRecord]
    public var nextCursor: String?
    public var next_cursor: String?
}

public struct ThreadStartResponse: Codable, Sendable {
    public struct Result: Codable, Sendable {
        public var thread: ThreadRecord?
    }
    public var result: Result?
    public var thread: ThreadRecord?
}

public struct ThreadResumeResponse: Codable, Sendable {
    public struct Result: Codable, Sendable {
        public var thread: ThreadRecord?
    }
    public var result: Result?
    public var thread: ThreadRecord?
}

public struct ThreadTokenUsage: Codable, Hashable, Sendable {
    public var total: TokenUsageBreakdown
    public var last: TokenUsageBreakdown
    public var modelContextWindow: Int?
}

public struct TokenUsageBreakdown: Codable, Hashable, Sendable {
    public var totalTokens: Int
    public var inputTokens: Int
    public var cachedInputTokens: Int
    public var outputTokens: Int
    public var reasoningOutputTokens: Int
}

public struct TurnPlanStep: Codable, Hashable, Sendable {
    public var step: String
    public var status: TurnPlanStepStatus
}

public enum TurnPlanStepStatus: String, Codable, Sendable {
    case pending
    case inProgress
    case completed
}

public struct TurnPlan: Codable, Hashable, Sendable {
    public var turnId: String
    public var explanation: String?
    public var steps: [TurnPlanStep]

    public init(turnId: String, explanation: String?, steps: [TurnPlanStep]) {
        self.turnId = turnId
        self.explanation = explanation
        self.steps = steps
    }
}

public struct ApprovalRequest: Codable, Hashable, Sendable, Identifiable {
    public var workspaceId: String
    public var requestId: Int
    public var method: String
    public var params: [String: JSONValue]

    public var id: String { "\(workspaceId)-\(requestId)" }

    public init(workspaceId: String, requestId: Int, method: String, params: [String: JSONValue]) {
        self.workspaceId = workspaceId
        self.requestId = requestId
        self.method = method
        self.params = params
    }

    enum CodingKeys: String, CodingKey {
        case workspaceId = "workspace_id"
        case requestId = "request_id"
        case method
        case params
    }
}

public struct RequestUserInputOption: Codable, Hashable, Sendable {
    public var label: String
    public var description: String
}

public struct RequestUserInputQuestion: Codable, Hashable, Sendable {
    public var id: String
    public var header: String
    public var question: String
    public var options: [RequestUserInputOption]?
}

public struct RequestUserInputParams: Codable, Hashable, Sendable {
    public var threadId: String
    public var turnId: String
    public var itemId: String
    public var questions: [RequestUserInputQuestion]

    enum CodingKeys: String, CodingKey {
        case threadId = "thread_id"
        case turnId = "turn_id"
        case itemId = "item_id"
        case questions
    }
}

public struct RequestUserInputRequest: Codable, Hashable, Sendable, Identifiable {
    public var workspaceId: String
    public var requestId: JSONValue
    public var params: RequestUserInputParams

    public var id: String { "\(workspaceId)-\(requestId.asString())" }

    enum CodingKeys: String, CodingKey {
        case workspaceId = "workspace_id"
        case requestId = "request_id"
        case params
    }
}

public struct RequestUserInputAnswer: Codable, Hashable, Sendable {
    public var answers: [String]
}

public struct RequestUserInputResponse: Codable, Hashable, Sendable {
    public var answers: [String: RequestUserInputAnswer]
}

public enum ApprovalDecision: String, Codable, Sendable {
    case accept
    case decline
}

public struct ReviewTarget: Codable, Hashable, Sendable {
    public var type: String
    public var branch: String?
    public var sha: String?
    public var title: String?
    public var instructions: String?

    public static func uncommittedChanges() -> ReviewTarget { .init(type: "uncommittedChanges", branch: nil, sha: nil, title: nil, instructions: nil) }
    public static func baseBranch(_ branch: String) -> ReviewTarget { .init(type: "baseBranch", branch: branch, sha: nil, title: nil, instructions: nil) }
    public static func commit(sha: String, title: String?) -> ReviewTarget { .init(type: "commit", branch: nil, sha: sha, title: title, instructions: nil) }
    public static func custom(_ instructions: String) -> ReviewTarget { .init(type: "custom", branch: nil, sha: nil, title: nil, instructions: instructions) }
}

public enum ReviewDelivery: String, Codable, Sendable {
    case inline
    case detached
}

// MARK: - Codex

public struct PingResponse: Codable, Sendable {
    public var ok: Bool
}

public struct CodexDoctorResult: Codable, Hashable, Sendable {
    public var ok: Bool
    public var codexBin: String?
    public var version: String?
    public var appServerOk: Bool
    public var details: String?
    public var path: String?
    public var nodeOk: Bool
    public var nodeVersion: String?
    public var nodeDetails: String?
}

// MARK: - Git

public struct GitFileStatus: Codable, Hashable, Sendable {
    public var path: String
    public var status: String
    public var additions: Int
    public var deletions: Int
}

public struct GitStatusResponse: Codable, Hashable, Sendable {
    public var branchName: String
    public var files: [GitFileStatus]
    public var stagedFiles: [GitFileStatus]
    public var unstagedFiles: [GitFileStatus]
    public var totalAdditions: Int
    public var totalDeletions: Int
}

public struct GitFileDiff: Codable, Hashable, Sendable {
    public var path: String
    public var diff: String
}

public struct GitCommitDiff: Codable, Hashable, Sendable {
    public var path: String
    public var status: String
    public var diff: String
}

public struct GitLogEntry: Codable, Hashable, Sendable {
    public var sha: String
    public var summary: String
    public var author: String
    public var timestamp: Double
}

public struct GitLogResponse: Codable, Hashable, Sendable {
    public var total: Int
    public var entries: [GitLogEntry]
    public var ahead: Int
    public var behind: Int
    public var aheadEntries: [GitLogEntry]
    public var behindEntries: [GitLogEntry]
    public var upstream: String?
}

public struct BranchInfo: Codable, Hashable, Sendable {
    public var name: String
    public var lastCommit: Double
}

// MARK: - GitHub

public struct GitHubIssue: Codable, Hashable, Sendable {
    public var number: Int
    public var title: String
    public var url: String
    public var updatedAt: String
}

public struct GitHubIssuesResponse: Codable, Hashable, Sendable {
    public var total: Int
    public var issues: [GitHubIssue]
}

public struct GitHubUser: Codable, Hashable, Sendable {
    public var login: String
}

public struct GitHubPullRequest: Codable, Hashable, Sendable {
    public var number: Int
    public var title: String
    public var url: String
    public var updatedAt: String
    public var createdAt: String
    public var body: String
    public var headRefName: String
    public var baseRefName: String
    public var isDraft: Bool
    public var author: GitHubUser?
}

public struct GitHubPullRequestsResponse: Codable, Hashable, Sendable {
    public var total: Int
    public var pullRequests: [GitHubPullRequest]

    enum CodingKeys: String, CodingKey {
        case total
        case pullRequests = "pullRequests"
    }
}

public struct GitHubPullRequestDiff: Codable, Hashable, Sendable {
    public var path: String
    public var status: String
    public var diff: String
}

public struct GitHubPullRequestComment: Codable, Hashable, Sendable {
    public var id: Int
    public var body: String
    public var createdAt: String
    public var url: String
    public var author: GitHubUser?
}

// MARK: - Usage

public struct LocalUsageDay: Codable, Hashable, Sendable {
    public var day: String
    public var inputTokens: Int
    public var cachedInputTokens: Int
    public var outputTokens: Int
    public var totalTokens: Int
    public var agentTimeMs: Int
    public var agentRuns: Int
}

public struct LocalUsageTotals: Codable, Hashable, Sendable {
    public var last7DaysTokens: Int
    public var last30DaysTokens: Int
    public var averageDailyTokens: Int
    public var cacheHitRatePercent: Double
    public var peakDay: String?
    public var peakDayTokens: Int
}

public struct LocalUsageModel: Codable, Hashable, Sendable {
    public var model: String
    public var tokens: Int
    public var sharePercent: Double
}

public struct LocalUsageSnapshot: Codable, Hashable, Sendable {
    public var updatedAt: Double
    public var days: [LocalUsageDay]
    public var totals: LocalUsageTotals
    public var topModels: [LocalUsageModel]
}

// MARK: - Prompt / Skills / Models

public struct ModelOption: Codable, Hashable, Sendable {
    public var id: String
    public var model: String
    public var displayName: String
    public var description: String
    public var supportedReasoningEfforts: [ReasoningEffortOption]
    public var defaultReasoningEffort: String
    public var isDefault: Bool
}

public struct ReasoningEffortOption: Codable, Hashable, Sendable {
    public var reasoningEffort: String
    public var description: String
}

public struct CollaborationModeOption: Codable, Hashable, Sendable {
    public var id: String
    public var label: String
    public var mode: String
    public var model: String
    public var reasoningEffort: String?
    public var developerInstructions: String?
    public var value: [String: JSONValue]
}

public enum PromptScope: String, Codable, Sendable {
    case workspace
    case global
}

public struct CustomPromptOption: Codable, Hashable, Sendable {
    public var name: String
    public var path: String
    public var description: String?
    public var argumentHint: String?
    public var content: String
    public var scope: PromptScope?

    enum CodingKeys: String, CodingKey {
        case name, path, description, content, scope
        case argumentHint = "argumentHint"
    }
}

// MARK: - Rate limits

public struct RateLimitWindow: Codable, Hashable, Sendable {
    public var usedPercent: Double
    public var windowDurationMins: Double?
    public var resetsAt: Double?
}

public struct CreditsSnapshot: Codable, Hashable, Sendable {
    public var hasCredits: Bool
    public var unlimited: Bool
    public var balance: String?
}

public struct RateLimitSnapshot: Codable, Hashable, Sendable {
    public var primary: RateLimitWindow?
    public var secondary: RateLimitWindow?
    public var credits: CreditsSnapshot?
    public var planType: String?
}

// MARK: - Dictation (optional)

public enum DictationModelState: String, Codable, Sendable {
    case missing
    case downloading
    case ready
    case error
}

public struct DictationDownloadProgress: Codable, Hashable, Sendable {
    public var totalBytes: Double?
    public var downloadedBytes: Double
}

public struct DictationModelStatus: Codable, Hashable, Sendable {
    public var state: DictationModelState
    public var modelId: String
    public var progress: DictationDownloadProgress?
    public var error: String?
    public var path: String?
}

public enum DictationSessionState: String, Codable, Sendable {
    case idle
    case listening
    case processing
}

public struct DictationEvent: Codable, Hashable, Sendable {
    public var type: String
    public var state: DictationSessionState?
    public var value: Double?
    public var text: String?
    public var message: String?
}

// MARK: - Terminal / Files

public struct TerminalSessionInfo: Codable, Hashable, Sendable {
    public var id: String
}

public struct WorkspaceFileResponse: Codable, Hashable, Sendable {
    public var content: String
    public var truncated: Bool
}
