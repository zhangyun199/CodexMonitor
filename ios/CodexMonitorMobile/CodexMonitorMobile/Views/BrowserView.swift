import SwiftUI

struct BrowserView: View {
    @EnvironmentObject private var store: CodexStore
    @Environment(\.scenePhase) private var scenePhase
    @State private var sessions: [String] = []
    @State private var selectedSession: String?
    @State private var url: String = ""
    @State private var screenshot: UIImage?
    @State private var screenshotWidth: CGFloat = 0
    @State private var screenshotHeight: CGFloat = 0
    @State private var autoRefresh = true
    @State private var refreshInterval: Double = 5

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 16) {
                    GlassSectionHeader(title: "Sessions", icon: "safari")

                    HStack {
                        Button("Refresh") { Task { await refreshSessions() } }
                        Button("New") {
                            Task {
                                if let created = await store.browserCreateSession() {
                                    selectedSession = created.sessionId
                                    await refreshSessions()
                                }
                            }
                        }
                    }

                    Picker("Session", selection: $selectedSession) {
                        ForEach(sessions, id: \.self) { session in
                            Text(session).tag(Optional(session))
                        }
                    }
                    .pickerStyle(.menu)

                    GlassSectionHeader(title: "Navigate", icon: "globe")
                    HStack {
                        TextField("URL", text: $url)
                            .textInputAutocapitalization(.never)
                            .keyboardType(.URL)
                        Button("Go") {
                            guard let sessionId = selectedSession else { return }
                            Task {
                                await store.browserNavigate(sessionId: sessionId, url: url)
                                await refreshScreenshot()
                            }
                        }
                    }

                    if let image = screenshot {
                        GlassSectionHeader(title: "Screenshot", icon: "photo")
                        GeometryReader { geo in
                            Image(uiImage: image)
                                .resizable()
                                .scaledToFit()
                                .frame(width: geo.size.width)
                                .gesture(
                                    DragGesture(minimumDistance: 0)
                                        .onEnded { value in
                                            guard let sessionId = selectedSession else { return }
                                            let scaleX = screenshotWidth > 0 ? screenshotWidth / geo.size.width : 1
                                            let scaleY = screenshotHeight > 0 ? screenshotHeight / geo.size.height : 1
                                            let x = value.location.x * scaleX
                                            let y = value.location.y * scaleY
                                            Task { await store.browserClick(sessionId: sessionId, x: x, y: y) }
                                        }
                                )
                        }
                        .frame(height: 300)
                    }

                    GlassSectionHeader(title: "Auto-Refresh", icon: "clock")
                    Toggle("Enabled", isOn: $autoRefresh)
                    Picker("Interval", selection: $refreshInterval) {
                        Text("3s").tag(3.0)
                        Text("5s").tag(5.0)
                        Text("10s").tag(10.0)
                    }
                    .pickerStyle(.segmented)
                }
                .padding()
            }
            .navigationTitle("Browser")
        }
        .task { await refreshSessions() }
        .task(id: "\(selectedSession ?? "")-\(autoRefresh)-\(refreshInterval)-\(scenePhase)") {
            guard autoRefresh, selectedSession != nil, scenePhase == .active else { return }
            while !Task.isCancelled {
                await refreshScreenshot()
                try? await Task.sleep(nanoseconds: UInt64(refreshInterval * 1_000_000_000))
            }
        }
    }

    private func refreshSessions() async {
        sessions = await store.browserListSessions()
        if selectedSession == nil {
            selectedSession = sessions.first
        }
    }

    private func refreshScreenshot() async {
        guard let sessionId = selectedSession else { return }
        if let shot = await store.browserScreenshot(sessionId: sessionId) {
            screenshot = decodeImage(shot.base64Png)
            screenshotWidth = CGFloat(shot.width ?? 0)
            screenshotHeight = CGFloat(shot.height ?? 0)
        }
    }

    private func decodeImage(_ base64: String) -> UIImage? {
        guard let data = Data(base64Encoded: base64) else { return nil }
        return UIImage(data: data)
    }
}
