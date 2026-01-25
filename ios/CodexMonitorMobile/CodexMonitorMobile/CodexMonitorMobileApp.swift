import SwiftUI

@main
struct CodexMonitorMobileApp: App {
    @StateObject private var store = CodexStore()
    @Environment(\.scenePhase) private var scenePhase

    var body: some Scene {
        WindowGroup {
            RootView()
                .environmentObject(store)
                .tint(Color(hex: "3DAAFF"))
                .onChange(of: scenePhase) { _, phase in
                    store.handleScenePhase(phase)
                }
        }
    }
}
