import SwiftUI

struct BrowserView: View {
    @EnvironmentObject private var store: CodexStore
    @State private var sessions: [String] = []
    @State private var selectedSession: String?
    @State private var url: String = ""
    @State private var screenshot: UIImage?
    @State private var screenshotWidth: CGFloat = 0
    @State private var screenshotHeight: CGFloat = 0

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
                }
                .padding()
            }
            .navigationTitle("Browser")
        }
        .task { await refreshSessions() }
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
