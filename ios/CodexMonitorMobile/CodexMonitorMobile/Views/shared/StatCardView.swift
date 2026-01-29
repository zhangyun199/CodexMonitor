import SwiftUI

struct StatCardView: View {
    var title: String
    var value: String

    var body: some View {
        GlassCardView {
            VStack(alignment: .leading, spacing: 6) {
                Text(title)
                    .font(.caption)
                    .foregroundStyle(.secondary)
                Text(value)
                    .font(.headline)
            }
        }
    }
}
