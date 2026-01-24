import SwiftUI
import CodexMonitorModels

struct RootView: View {
    @EnvironmentObject private var store: CodexStore
    @Environment(\.horizontalSizeClass) private var horizontalSizeClass
    @State private var showSettings = false

    var body: some View {
        Group {
            if horizontalSizeClass == .compact {
                PhoneRootView(showSettings: $showSettings)
            } else {
                TabletRootView(showSettings: $showSettings)
            }
        }
        .sheet(isPresented: $showSettings) {
            SettingsView()
        }
        .onAppear {
            if store.connectionState == .disconnected {
                store.connect()
            }
        }
    }
}

private struct PhoneRootView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var showSettings: Bool
    @AppStorage("themeGradient") private var themeGradient = ThemeGradient.midnightBlue

    var body: some View {
        ZStack {
            // Background gradient
            themeGradient.gradient
                .ignoresSafeArea()

            TabView {
                NavigationStack {
                    ProjectsView()
                        .toolbar {
                            ToolbarItem(placement: .topBarTrailing) {
                                Button(action: { showSettings = true }) {
                                    Image(systemName: "gearshape")
                                }
                            }
                        }
                }
                .tabItem {
                    Label("Projects", systemImage: "folder")
                }

                NavigationStack {
                    ConversationTabView()
                        .toolbar {
                            ToolbarItem(placement: .topBarTrailing) {
                                Button(action: { showSettings = true }) {
                                    Image(systemName: "gearshape")
                                }
                            }
                        }
                }
                .tabItem {
                    Label("Codex", systemImage: "bubble.left.and.text.bubble.right")
                }

                NavigationStack {
                    GitView()
                        .toolbar {
                            ToolbarItem(placement: .topBarTrailing) {
                                Button(action: { showSettings = true }) {
                                    Image(systemName: "gearshape")
                                }
                            }
                        }
                }
                .tabItem {
                    Label("Git", systemImage: "arrow.triangle.branch")
                }

                NavigationStack {
                    DebugLogView()
                        .toolbar {
                            ToolbarItem(placement: .topBarTrailing) {
                                Button(action: { showSettings = true }) {
                                    Image(systemName: "gearshape")
                                }
                            }
                        }
                }
                .tabItem {
                    Label("Log", systemImage: "waveform.path.ecg")
                }
            }
            .background(.clear)
        }
    }
}

private struct TabletRootView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var showSettings: Bool
    @State private var selectedWorkspace: WorkspaceInfo?
    @State private var selectedThreadId: String?
    @State private var detailSelection: TabletDetail = .conversation
    @AppStorage("themeGradient") private var themeGradient = ThemeGradient.midnightBlue

    enum TabletDetail: String, CaseIterable {
        case conversation = "Conversation"
        case git = "Git"
        case files = "Files"
        case prompts = "Prompts"
        case terminal = "Terminal"
    }

    var body: some View {
        ZStack {
            // Background gradient
            themeGradient.gradient
                .ignoresSafeArea()

            NavigationSplitView {
                WorkspaceListView(selectedWorkspace: $selectedWorkspace)
                    .navigationTitle("Workspaces")
            } content: {
                ThreadsListView(
                    selectedWorkspace: $selectedWorkspace,
                    selectedThreadId: $selectedThreadId
                )
                .navigationTitle("Threads")
            } detail: {
                VStack(spacing: 0) {
                    Picker("Detail", selection: $detailSelection) {
                        ForEach(TabletDetail.allCases, id: \.self) { item in
                            Text(item.rawValue).tag(item)
                        }
                    }
                    .pickerStyle(.segmented)
                    .padding()

                    Group {
                        switch detailSelection {
                        case .conversation:
                            ConversationTabView(selectedThreadId: selectedThreadId)
                        case .git:
                            GitView()
                        case .files:
                            FilesView()
                        case .prompts:
                            PromptsView()
                        case .terminal:
                            TerminalView()
                        }
                    }
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                }
                .toolbar {
                    ToolbarItem(placement: .topBarTrailing) {
                        Button(action: { showSettings = true }) {
                            Image(systemName: "gearshape")
                        }
                    }
                }
            }
            .navigationSplitViewStyle(.balanced)
            .background(.clear)
        }
    }
}
