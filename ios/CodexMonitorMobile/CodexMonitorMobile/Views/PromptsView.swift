import SwiftUI
import UIKit
import CodexMonitorModels

struct PromptsView: View {
    @EnvironmentObject private var store: CodexStore
    @State private var showCreate = false

    var body: some View {
        VStack {
            if let workspaceId = store.activeWorkspaceId {
                List {
                    ForEach(store.promptsByWorkspace[workspaceId] ?? [], id: \.path) { prompt in
                        VStack(alignment: .leading, spacing: 6) {
                            Text(prompt.name)
                                .font(.headline)
                            if let description = prompt.description {
                                Text(description)
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                            Text(prompt.content)
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                                .lineLimit(3)
                        }
                        .contextMenu {
                            Button("Copy Prompt") {
                                UIPasteboard.general.string = prompt.content
                            }
                            Button("Delete", role: .destructive) {
                                Task { await store.deletePrompt(workspaceId: workspaceId, path: prompt.path) }
                            }
                        }
                    }
                }
                .task { await store.refreshPrompts(workspaceId: workspaceId) }
                .refreshable { await store.refreshPrompts(workspaceId: workspaceId) }
                .toolbar {
                    ToolbarItem(placement: .topBarTrailing) {
                        Button(action: { showCreate = true }) {
                            Image(systemName: "plus")
                        }
                    }
                }
                .sheet(isPresented: $showCreate) {
                    CreatePromptSheet()
                }
            } else {
                ContentUnavailableView("No workspace selected", systemImage: "text.badge.plus")
            }
        }
        .navigationTitle("Prompts")
    }
}

private struct CreatePromptSheet: View {
    @EnvironmentObject private var store: CodexStore
    @Environment(\.dismiss) private var dismiss
    @State private var name = ""
    @State private var description = ""
    @State private var argumentHint = ""
    @State private var content = ""
    @State private var scope: PromptScope = .workspace

    var body: some View {
        NavigationStack {
            Form {
                Section("Name") {
                    TextField("Prompt name", text: $name)
                }
                Section("Description") {
                    TextField("Optional description", text: $description)
                }
                Section("Argument hint") {
                    TextField("Optional argument hint", text: $argumentHint)
                }
                Section("Content") {
                    TextEditor(text: $content)
                        .frame(minHeight: 120)
                }
                Section("Scope") {
                    Picker("Scope", selection: $scope) {
                        Text("Workspace").tag(PromptScope.workspace)
                        Text("Global").tag(PromptScope.global)
                    }
                    .pickerStyle(.segmented)
                }
            }
            .navigationTitle("New Prompt")
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Save") {
                        guard let workspaceId = store.activeWorkspaceId else { return }
                        Task {
                            await store.createPrompt(
                                workspaceId: workspaceId,
                                scope: scope,
                                name: name,
                                description: description.isEmpty ? nil : description,
                                argumentHint: argumentHint.isEmpty ? nil : argumentHint,
                                content: content
                            )
                            dismiss()
                        }
                    }
                    .disabled(name.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty || content.isEmpty)
                }
            }
        }
    }
}
