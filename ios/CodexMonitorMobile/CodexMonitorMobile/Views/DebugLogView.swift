import SwiftUI

struct DebugLogView: View {
    @EnvironmentObject private var store: CodexStore

    var body: some View {
        List {
            ForEach(store.debugEntries) { entry in
                VStack(alignment: .leading, spacing: 4) {
                    Text(entry.label)
                        .font(.headline)
                    Text(entry.timestamp, style: .time)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                    if let payload = entry.payload {
                        Text(payload.asString())
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                }
            }
        }
        .scrollContentBackground(.hidden)
        .background {
            GradientBackground()
        }
        .navigationTitle("Debug Log")
        .toolbar {
            ToolbarItem(placement: .topBarTrailing) {
                Button("Clear") { store.debugEntries.removeAll() }
            }
        }
    }
}
