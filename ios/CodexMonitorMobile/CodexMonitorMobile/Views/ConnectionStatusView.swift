import SwiftUI

struct ConnectionStatusView: View {
    @EnvironmentObject private var store: CodexStore

    var body: some View {
        GlassCard {
            HStack {
                statusIndicator
                VStack(alignment: .leading, spacing: 4) {
                    Text(statusTitle)
                        .font(.headline)
                    if let detail = statusDetail {
                        Text(detail)
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                }
                Spacer()
                Button(action: { store.connect() }) {
                    Image(systemName: "arrow.clockwise")
                }
            }
        }
    }

    private var statusTitle: String {
        switch store.connectionState {
        case .connected: return "Connected"
        case .connecting: return "Connecting…"
        case .disconnected: return "Disconnected"
        case .error: return "Connection Error"
        }
    }

    private var statusDetail: String? {
        switch store.connectionState {
        case .error(let message):
            return message
        default:
            return nil
        }
    }

    private var statusColor: Color {
        switch store.connectionState {
        case .connected: return .green
        case .connecting: return .orange
        case .disconnected: return .gray
        case .error: return .red
        }
    }

    private var statusIndicator: some View {
        Circle()
            .fill(statusColor)
            .frame(width: 10, height: 10)
    }
}
