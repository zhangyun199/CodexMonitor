import SwiftUI

// MARK: - Glass Card
struct GlassCard<Content: View>: View {
    @Namespace private var glassNamespace
    let id: String
    @ViewBuilder var content: Content

    init(id: String = UUID().uuidString, @ViewBuilder content: () -> Content) {
        self.id = id
        self.content = content()
    }

    var body: some View {
        if #available(iOS 26.0, *) {
            content
                .padding(14)
                .glassEffect(.regular, in: .rect(cornerRadius: 18))
                .overlay(
                    RoundedRectangle(cornerRadius: 18)
                        .strokeBorder(.white.opacity(0.18), lineWidth: 1)
                )
                .glassEffectID(id, in: glassNamespace)
        } else {
            content
                .padding(14)
                .background(.ultraThinMaterial, in: RoundedRectangle(cornerRadius: 18))
                .overlay(
                    RoundedRectangle(cornerRadius: 18)
                        .strokeBorder(.white.opacity(0.18), lineWidth: 1)
                )
        }
    }
}

// MARK: - Glass Group
struct GlassGroup<Content: View>: View {
    @ViewBuilder var content: Content

    init(@ViewBuilder content: () -> Content) {
        self.content = content()
    }

    var body: some View {
        if #available(iOS 26.0, *) {
            GlassEffectContainer {
                content
            }
        } else {
            content
        }
    }
}

// MARK: - Glass Badge
struct GlassBadge: View {
    let text: String

    var body: some View {
        if #available(iOS 26.0, *) {
            Text(text)
                .font(.caption.weight(.semibold))
                .padding(.horizontal, 10)
                .padding(.vertical, 6)
                .glassEffect(.regular, in: .capsule)
        } else {
            Text(text)
                .font(.caption.weight(.semibold))
                .padding(.horizontal, 10)
                .padding(.vertical, 6)
                .background(.ultraThinMaterial, in: Capsule())
        }
    }
}

// MARK: - Glass Chip
struct GlassChip: View {
    let text: String
    var tint: Color = .blue

    var body: some View {
        if #available(iOS 26.0, *) {
            Text(text)
                .font(.caption.weight(.medium))
                .padding(.horizontal, 12)
                .padding(.vertical, 6)
                .glassEffect(.regular.tint(tint).interactive(), in: .capsule)
        } else {
            Text(text)
                .font(.caption.weight(.medium))
                .foregroundStyle(tint)
                .padding(.horizontal, 12)
                .padding(.vertical, 6)
                .background(.ultraThinMaterial, in: Capsule())
        }
    }
}
