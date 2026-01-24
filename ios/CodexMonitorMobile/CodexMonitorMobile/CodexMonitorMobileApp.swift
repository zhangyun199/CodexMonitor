import SwiftUI

@main
struct CodexMonitorMobileApp: App {
    @StateObject private var store = CodexStore()
    @Environment(\.scenePhase) private var scenePhase

    var body: some Scene {
        WindowGroup {
            RootView()
                .environmentObject(store)
                .onChange(of: scenePhase) { _, phase in
                    store.handleScenePhase(phase)
                }
        }
    }
}
