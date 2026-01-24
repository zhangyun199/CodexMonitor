import SwiftUI

struct TerminalView: View {
    @EnvironmentObject private var store: CodexStore
    @State private var terminalId: String = UUID().uuidString
    @State private var input = ""

    var body: some View {
        GeometryReader { proxy in
            VStack(spacing: 8) {
                if let workspaceId = store.activeWorkspaceId {
                    TextEditor(text: Binding(
                        get: { store.terminalOutputBySession[sessionKey(workspaceId)] ?? "" },
                        set: { _ in }
                    ))
                    .font(.system(.footnote, design: .monospaced))
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .overlay(RoundedRectangle(cornerRadius: 8).stroke(Color.secondary.opacity(0.2)))

                    HStack {
                        TextField("Command", text: $input)
                            .textInputAutocapitalization(.never)
                            .autocorrectionDisabled()
                        Button("Send") {
                            let payload = input + "\n"
                            input = ""
                            Task { await store.writeTerminal(workspaceId: workspaceId, terminalId: terminalId, data: payload) }
                        }
                        .buttonStyle(.borderedProminent)
                    }
                } else {
                    ContentUnavailableView("No workspace selected", systemImage: "terminal")
                }
            }
            .onChange(of: proxy.size) { _, newSize in
                if let workspaceId = store.activeWorkspaceId {
                    let (cols, rows) = estimateTerminalSize(size: newSize)
                    Task { await store.resizeTerminal(workspaceId: workspaceId, terminalId: terminalId, cols: cols, rows: rows) }
                }
            }
        }
        .padding()
        .background {
            GradientBackground()
        }
        .navigationTitle("Terminal")
        .task {
            if let workspaceId = store.activeWorkspaceId {
                await store.openTerminal(workspaceId: workspaceId, terminalId: terminalId, cols: 120, rows: 40)
            }
        }
        .onDisappear {
            if let workspaceId = store.activeWorkspaceId {
                Task { await store.closeTerminal(workspaceId: workspaceId, terminalId: terminalId) }
            }
        }
    }

    private func sessionKey(_ workspaceId: String) -> String {
        "\(workspaceId)-\(terminalId)"
    }

    private func estimateTerminalSize(size: CGSize) -> (Int, Int) {
        let colWidth: CGFloat = 8.0
        let rowHeight: CGFloat = 16.0
        let cols = max(20, Int(size.width / colWidth))
        let rows = max(10, Int(size.height / rowHeight))
        return (cols, rows)
    }
}
