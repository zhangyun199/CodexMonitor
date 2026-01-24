import SwiftUI

enum ThemeGradient: String, CaseIterable, Codable {
    case midnightBlue
    case oceanDeep
    case cosmicPurple
    case slateDark

    var displayName: String {
        switch self {
        case .midnightBlue: return "Midnight Blue"
        case .oceanDeep: return "Ocean Deep"
        case .cosmicPurple: return "Cosmic Purple"
        case .slateDark: return "Slate Dark"
        }
    }

    var gradient: LinearGradient {
        switch self {
        case .midnightBlue:
            return LinearGradient(
                colors: [Color(hex: "0f0c29"), Color(hex: "302b63"), Color(hex: "24243e")],
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
        case .oceanDeep:
            return LinearGradient(
                colors: [Color(hex: "0d1b2a"), Color(hex: "1b263b"), Color(hex: "415a77")],
                startPoint: .top,
                endPoint: .bottom
            )
        case .cosmicPurple:
            return LinearGradient(
                colors: [Color(hex: "10002b"), Color(hex: "240046"), Color(hex: "3c096c")],
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
        case .slateDark:
            return LinearGradient(
                colors: [Color(hex: "1a1a1a"), Color(hex: "2d2d2d"), Color(hex: "1f1f1f")],
                startPoint: .top,
                endPoint: .bottom
            )
        }
    }

    var previewColors: [Color] {
        switch self {
        case .midnightBlue: return [Color(hex: "0f0c29"), Color(hex: "302b63")]
        case .oceanDeep: return [Color(hex: "0d1b2a"), Color(hex: "415a77")]
        case .cosmicPurple: return [Color(hex: "10002b"), Color(hex: "3c096c")]
        case .slateDark: return [Color(hex: "1a1a1a"), Color(hex: "2d2d2d")]
        }
    }
}
