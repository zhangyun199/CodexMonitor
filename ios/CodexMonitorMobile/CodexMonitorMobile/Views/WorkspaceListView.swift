import SwiftUI
import CodexMonitorModels

struct WorkspaceListView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var selectedWorkspace: WorkspaceInfo?
    @State private var showAddWorkspace = false

    var body: some View {
        List(selection: $selectedWorkspace) {
            ForEach(store.workspaces) { workspace in
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text(workspace.name)
                            .font(.headline)
                        Text(workspace.path)
                            .font(.caption)
                            .foregroundStyle(.secondary)
                            .lineLimit(1)
                    }
                    Spacer()
                    Circle()
                        .fill(workspace.connected ? Color.green : Color.orange)
                        .frame(width: 10, height: 10)
                }
                .contentShape(Rectangle())
                .onTapGesture {
                    selectedWorkspace = workspace
                    store.activeWorkspaceId = workspace.id
                }
            }
        }
        .toolbar {
            ToolbarItem(placement: .topBarTrailing) {
                Button(action: { showAddWorkspace = true }) {
                    Image(systemName: "plus")
                }
            }
            ToolbarItem(placement: .topBarLeading) {
                Button(action: { Task { await store.refreshWorkspaces() } }) {
                    Image(systemName: "arrow.clockwise")
                }
            }
        }
        .sheet(isPresented: $showAddWorkspace) {
            AddWorkspaceSheet()
        }
    }
}

private struct AddWorkspaceSheet: View {
    @EnvironmentObject private var store: CodexStore
    @Environment(\.dismiss) private var dismiss
    @State private var path = ""
    @State private var codexBin = ""

    var body: some View {
        NavigationStack {
            Form {
                Section(header: Text("Workspace Path")) {
                    TextField("/path/to/repo", text: $path)
                        .textInputAutocapitalization(.never)
                        .autocorrectionDisabled()
                }
                Section(header: Text("Optional codex bin")) {
                    TextField("codex bin path", text: $codexBin)
                        .textInputAutocapitalization(.never)
                        .autocorrectionDisabled()
                }
            }
            .navigationTitle("Add Workspace")
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Add") {
                        Task {
                            await store.addWorkspace(path: path, codexBin: codexBin.isEmpty ? nil : codexBin)
                            dismiss()
                        }
                    }
                    .disabled(path.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
                }
            }
        }
    }
}
