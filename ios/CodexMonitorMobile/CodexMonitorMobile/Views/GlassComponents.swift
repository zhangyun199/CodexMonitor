import SwiftUI

struct GlassCard<Content: View>: View {
    let id: String
    @ViewBuilder var content: Content

    init(id: String = UUID().uuidString, @ViewBuilder content: () -> Content) {
        self.id = id
        self.content = content()
    }

    var body: some View {
        content
            .padding(14)
            .glassEffect(.regular, in: .rect(cornerRadius: 18))
            .overlay(
                RoundedRectangle(cornerRadius: 18)
                    .strokeBorder(.white.opacity(0.18), lineWidth: 1)
            )
            .glassEffectID(id)
    }
}

struct GlassGroup<Content: View>: View {
    @ViewBuilder var content: Content

    init(@ViewBuilder content: () -> Content) {
        self.content = content()
    }

    var body: some View {
        GlassEffectContainer {
            content
        }
    }
}

struct GlassBadge: View {
    let text: String

    var body: some View {
        Text(text)
            .font(.caption.weight(.semibold))
            .padding(.horizontal, 10)
            .padding(.vertical, 6)
            .glassEffect(.regular, in: .capsule)
    }
}

struct GlassChip: View {
    let text: String
    var tint: Color = .blue

    var body: some View {
        Text(text)
            .font(.caption.weight(.medium))
            .padding(.horizontal, 12)
            .padding(.vertical, 6)
            .glassEffect(.regular.tint(tint).interactive(), in: .capsule)
    }
}
