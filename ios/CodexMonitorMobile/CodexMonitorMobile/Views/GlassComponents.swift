import SwiftUI

// MARK: - Raised Glass Style
struct RaisedGlassStyle: ViewModifier {
    var cornerRadius: CGFloat = 14
    var tint: Color? = nil
    var colorBoost: Double = 0.12
    var borderOpacity: Double = 0.22
    var interactive: Bool = false
    var lift: CGFloat = 4

    private var overlayTint: Color {
        if let tint {
            return tint.opacity(colorBoost)
        } else {
            return Color.white.opacity(colorBoost * 0.6)
        }
    }

    private var borderTint: Color {
        if let tint {
            return tint.opacity(borderOpacity)
        } else {
            return Color.white.opacity(borderOpacity)
        }
    }

    private var glowShadow: Color {
        if let tint {
            return tint.opacity(0.28)
        } else {
            return Color.black.opacity(0.18)
        }
    }

    func body(content: Content) -> some View {
        if #available(iOS 26.0, *) {
            content
                .background {
                    ZStack {
                        RoundedRectangle(cornerRadius: cornerRadius)
                            .fill(.clear)
                            .glassEffect(
                                tint != nil
                                    ? (interactive
                                        ? .regular.tint(tint!).interactive()
                                        : .regular.tint(tint!))
                                    : (interactive
                                        ? .regular.interactive()
                                        : .regular),
                                in: .rect(cornerRadius: cornerRadius)
                            )

                        RoundedRectangle(cornerRadius: cornerRadius)
                            .fill(overlayTint)
                            .blendMode(.plusLighter)
                    }
                }
                .overlay {
                    ZStack {
                        RoundedRectangle(cornerRadius: cornerRadius)
                            .strokeBorder(Color.white.opacity(0.18), lineWidth: 0.5)
                        RoundedRectangle(cornerRadius: cornerRadius)
                            .strokeBorder(borderTint, lineWidth: 0.9)
                    }
                }
                .shadow(color: glowShadow, radius: 10, x: 0, y: lift + 2)
                .shadow(color: Color.black.opacity(0.22), radius: 8, x: 0, y: lift)
        } else {
            content
                .background(
                    RoundedRectangle(cornerRadius: cornerRadius)
                        .fill(overlayTint)
                        .background(.ultraThinMaterial, in: RoundedRectangle(cornerRadius: cornerRadius))
                )
                .overlay {
                    ZStack {
                        RoundedRectangle(cornerRadius: cornerRadius)
                            .strokeBorder(Color.white.opacity(0.18), lineWidth: 0.5)
                        RoundedRectangle(cornerRadius: cornerRadius)
                            .strokeBorder(borderTint, lineWidth: 0.9)
                    }
                }
                .shadow(color: glowShadow, radius: 8, x: 0, y: lift + 1)
                .shadow(color: Color.black.opacity(0.2), radius: 6, x: 0, y: lift)
        }
    }
}

extension View {
    func raisedGlassStyle(
        cornerRadius: CGFloat = 14,
        tint: Color? = nil,
        colorBoost: Double = 0.12,
        borderOpacity: Double = 0.22,
        interactive: Bool = false,
        lift: CGFloat = 4
    ) -> some View {
        modifier(
            RaisedGlassStyle(
                cornerRadius: cornerRadius,
                tint: tint,
                colorBoost: colorBoost,
                borderOpacity: borderOpacity,
                interactive: interactive,
                lift: lift
            )
        )
    }
}

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

// MARK: - Glass Icon Button
struct GlassIconButton: View {
    let icon: String
    var tint: Color = .primary
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            if #available(iOS 26.0, *) {
                Image(systemName: icon)
                    .font(.system(size: 16, weight: .medium))
                    .foregroundStyle(tint)
                    .frame(width: 36, height: 36)
                    .glassEffect(.regular.interactive(), in: .circle)
            } else {
                Image(systemName: icon)
                    .font(.system(size: 16, weight: .medium))
                    .foregroundStyle(tint)
                    .frame(width: 36, height: 36)
                    .background(.ultraThinMaterial, in: Circle())
            }
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Glass Section Header
struct GlassSectionHeader: View {
    let title: String
    var icon: String? = nil

    var body: some View {
        HStack(spacing: 8) {
            if let icon {
                Image(systemName: icon)
                    .font(.system(size: 12, weight: .semibold))
            }
            Text(title)
                .font(.subheadline.weight(.semibold))
                .textCase(.uppercase)
                .tracking(0.5)
        }
        .foregroundStyle(.secondary)
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(.horizontal, 4)
        .padding(.vertical, 8)
    }
}

// MARK: - Glass Panel
struct GlassPanel<Content: View>: View {
    var cornerRadius: CGFloat = 16
    @ViewBuilder var content: Content

    init(cornerRadius: CGFloat = 16, @ViewBuilder content: () -> Content) {
        self.cornerRadius = cornerRadius
        self.content = content()
    }

    var body: some View {
        if #available(iOS 26.0, *) {
            content
                .glassEffect(.regular, in: .rect(cornerRadius: cornerRadius))
                .overlay(
                    RoundedRectangle(cornerRadius: cornerRadius)
                        .strokeBorder(.white.opacity(0.12), lineWidth: 0.5)
                )
        } else {
            content
                .background(.ultraThinMaterial, in: RoundedRectangle(cornerRadius: cornerRadius))
                .overlay(
                    RoundedRectangle(cornerRadius: cornerRadius)
                        .strokeBorder(.white.opacity(0.12), lineWidth: 0.5)
                )
        }
    }
}

// MARK: - Glass List Row Modifier
struct GlassListRowModifier: ViewModifier {
    let isSelected: Bool
    var cornerRadius: CGFloat = 12

    func body(content: Content) -> some View {
        content
            .raisedGlassStyle(
                cornerRadius: cornerRadius,
                tint: isSelected ? .accentColor : nil,
                colorBoost: isSelected ? 0.12 : 0.06,
                borderOpacity: isSelected ? 0.45 : 0.2,
                interactive: isSelected,
                lift: isSelected ? 5 : 4
            )
    }
}

extension View {
    func glassListRow(isSelected: Bool, cornerRadius: CGFloat = 12) -> some View {
        modifier(GlassListRowModifier(isSelected: isSelected, cornerRadius: cornerRadius))
    }
}
