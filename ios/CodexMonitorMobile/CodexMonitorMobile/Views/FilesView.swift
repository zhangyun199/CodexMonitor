import SwiftUI
import CodexMonitorRendering
import CodexMonitorModels

struct FilesView: View {
    @EnvironmentObject private var store: CodexStore
    @State private var searchText = ""
    @State private var selectedFile: String?
    @State private var selectedContent: WorkspaceFileResponse?

    var body: some View {
        VStack {
            if let workspaceId = store.activeWorkspaceId {
                List(filteredFiles(for: workspaceId), id: \.self) { path in
                    Text(path)
                        .font(.subheadline)
                        .onTapGesture {
                            selectedFile = path
                            Task {
                                selectedContent = await store.readFile(workspaceId: workspaceId, path: path)
                            }
                        }
                }
                .scrollContentBackground(.hidden)
                .background {
                    GradientBackground()
                }
                .searchable(text: $searchText)
                .task {
                    await store.refreshFiles(workspaceId: workspaceId)
                }
                .refreshable {
                    await store.refreshFiles(workspaceId: workspaceId)
                }
                .sheet(isPresented: Binding(
                    get: { selectedFile != nil },
                    set: { if !$0 { selectedFile = nil } }
                )) {
                    if let content = selectedContent {
                        NavigationStack {
                            ScrollView {
                                Text(CodexMonitorRendering.monospaced(content.content))
                                    .padding()
                            }
                            .navigationTitle(selectedFile ?? "File")
                            .toolbar {
                                ToolbarItem(placement: .cancellationAction) {
                                    Button("Close") { selectedFile = nil }
                                }
                            }
                        }
                    }
                }
            } else {
                ContentUnavailableView("No workspace selected", systemImage: "doc.text.magnifyingglass")
            }
        }
        .navigationTitle("Files")
    }

    private func filteredFiles(for workspaceId: String) -> [String] {
        let files = store.filesByWorkspace[workspaceId] ?? []
        guard !searchText.isEmpty else { return files }
        return files.filter { $0.localizedCaseInsensitiveContains(searchText) }
    }
}
