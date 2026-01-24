import SwiftUI
import CodexMonitorRendering
import CodexMonitorModels

struct GitView: View {
    @EnvironmentObject private var store: CodexStore
    @State private var commitMessage = ""
    @State private var selectedDiff: GitFileDiff?
    @State private var tab: GitTab = .status
    @State private var selectedPR: GitHubPullRequest?
    @State private var prDiffs: [GitHubPullRequestDiff] = []
    @State private var prComments: [GitHubPullRequestComment] = []

    enum GitTab: String, CaseIterable {
        case status = "Status"
        case issues = "Issues"
        case pullRequests = "PRs"
    }

    var body: some View {
        VStack {
            if let workspaceId = store.activeWorkspaceId {
                Picker("Tab", selection: $tab) {
                    ForEach(GitTab.allCases, id: \.self) { item in
                        Text(item.rawValue).tag(item)
                    }
                }
                .pickerStyle(.segmented)
                .padding([.horizontal, .top])

                List {
                    switch tab {
                    case .status:
                        Section("Status") {
                            Text("Branch: \(store.gitStatusByWorkspace[workspaceId]?.branchName ?? "—")")
                                .font(.headline)
                            HStack {
                                Button("Stage All") {
                                    Task { await store.stageAll(workspaceId: workspaceId) }
                                }
                                Button("Revert All") {
                                    Task { await store.revertAll(workspaceId: workspaceId) }
                                }
                            }
                        }

                        Section("Unstaged") {
                            ForEach(store.gitStatusByWorkspace[workspaceId]?.unstagedFiles ?? [], id: \.path) { file in
                                GitFileRow(file: file, actionLabel: "Stage") {
                                    Task { await store.stageGitFile(workspaceId: workspaceId, path: file.path) }
                                }
                                .onTapGesture {
                                    selectedDiff = store.gitDiffsByWorkspace[workspaceId]?.first { $0.path == file.path }
                                }
                            }
                        }

                        Section("Staged") {
                            ForEach(store.gitStatusByWorkspace[workspaceId]?.stagedFiles ?? [], id: \.path) { file in
                                GitFileRow(file: file, actionLabel: "Unstage") {
                                    Task { await store.unstageGitFile(workspaceId: workspaceId, path: file.path) }
                                }
                                .onTapGesture {
                                    selectedDiff = store.gitDiffsByWorkspace[workspaceId]?.first { $0.path == file.path }
                                }
                            }
                        }

                        Section("Commit") {
                            TextField("Commit message", text: $commitMessage)
                            Button("Generate Message") {
                                Task {
                                    if let message = await store.generateCommitMessage(workspaceId: workspaceId) {
                                        commitMessage = message
                                    }
                                }
                            }
                            Button("Commit") {
                                let trimmed = commitMessage.trimmingCharacters(in: .whitespacesAndNewlines)
                                guard !trimmed.isEmpty else { return }
                                Task {
                                    await store.commit(workspaceId: workspaceId, message: trimmed)
                                    commitMessage = ""
                                }
                            }
                            .buttonStyle(.borderedProminent)
                        }

                        Section("Remote") {
                            HStack {
                                Button("Pull") { Task { await store.pull(workspaceId: workspaceId) } }
                                Button("Push") { Task { await store.push(workspaceId: workspaceId) } }
                                Button("Sync") { Task { await store.sync(workspaceId: workspaceId) } }
                            }
                        }
                    case .issues:
                        Section("Issues") {
                            ForEach(store.gitHubIssuesByWorkspace[workspaceId] ?? [], id: \.number) { issue in
                                VStack(alignment: .leading) {
                                    Text("#\(issue.number) \(issue.title)")
                                        .font(.headline)
                                    Text(issue.updatedAt)
                                        .font(.caption2)
                                        .foregroundStyle(.secondary)
                                }
                            }
                        }
                    case .pullRequests:
                        Section("Pull Requests") {
                            ForEach(store.gitHubPullsByWorkspace[workspaceId] ?? [], id: \.number) { pr in
                                VStack(alignment: .leading) {
                                    Text("#\(pr.number) \(pr.title)")
                                        .font(.headline)
                                    Text(pr.updatedAt)
                                        .font(.caption2)
                                        .foregroundStyle(.secondary)
                                }
                                .onTapGesture {
                                    selectedPR = pr
                                    Task {
                                        prDiffs = await store.fetchPullRequestDiff(workspaceId: workspaceId, prNumber: pr.number)
                                        prComments = await store.fetchPullRequestComments(workspaceId: workspaceId, prNumber: pr.number)
                                    }
                                }
                            }
                        }
                    }
                }
                .scrollContentBackground(.hidden)
                .background(.clear)
                .refreshable {
                    switch tab {
                    case .status:
                        await store.refreshGitStatus(workspaceId: workspaceId)
                    case .issues:
                        await store.refreshGitHubIssues(workspaceId: workspaceId)
                    case .pullRequests:
                        await store.refreshGitHubPulls(workspaceId: workspaceId)
                    }
                }
                .task {
                    await store.refreshGitStatus(workspaceId: workspaceId)
                }
                .onChange(of: tab) { _, newValue in
                    Task {
                        switch newValue {
                        case .status:
                            await store.refreshGitStatus(workspaceId: workspaceId)
                        case .issues:
                            await store.refreshGitHubIssues(workspaceId: workspaceId)
                        case .pullRequests:
                            await store.refreshGitHubPulls(workspaceId: workspaceId)
                        }
                    }
                }
                .sheet(isPresented: Binding(
                    get: { selectedDiff != nil },
                    set: { if !$0 { selectedDiff = nil } }
                )) {
                    if let diff = selectedDiff {
                        NavigationStack {
                            ScrollView {
                                Text(CodexMonitorRendering.monospaced(diff.diff))
                                    .padding()
                            }
                            .navigationTitle(diff.path)
                            .toolbar {
                                ToolbarItem(placement: .cancellationAction) {
                                    Button("Close") { selectedDiff = nil }
                                }
                            }
                        }
                    }
                }
                .sheet(isPresented: Binding(
                    get: { selectedPR != nil },
                    set: { if !$0 { selectedPR = nil } }
                )) {
                    if let pr = selectedPR {
                        NavigationStack {
                            List {
                                Section("Diff") {
                                    ForEach(prDiffs, id: \.path) { diff in
                                        VStack(alignment: .leading) {
                                            Text(diff.path).font(.headline)
                                            Text(CodexMonitorRendering.monospaced(diff.diff))
                                                .font(.caption2)
                                                .foregroundStyle(.secondary)
                                        }
                                    }
                                }
                                Section("Comments") {
                                    ForEach(prComments, id: \.id) { comment in
                                        VStack(alignment: .leading, spacing: 4) {
                                            Text(comment.author?.login ?? "unknown")
                                                .font(.caption.weight(.semibold))
                                            Text(comment.body)
                                                .font(.caption)
                                        }
                                    }
                                }
                            }
                            .navigationTitle("PR #\(pr.number)")
                            .toolbar {
                                ToolbarItem(placement: .cancellationAction) {
                                    Button("Close") { selectedPR = nil }
                                }
                            }
                        }
                    }
                }
            } else {
                ContentUnavailableView("No workspace selected", systemImage: "arrow.triangle.branch")
            }
        }
        .navigationTitle("Git")
    }
}

private struct GitFileRow: View {
    let file: GitFileStatus
    let actionLabel: String
    let action: () -> Void

    var body: some View {
        HStack {
            VStack(alignment: .leading) {
                Text(file.path)
                    .font(.subheadline)
                Text(file.status.uppercased())
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
            Spacer()
            Button(actionLabel, action: action)
                .buttonStyle(.bordered)
        }
    }
}
