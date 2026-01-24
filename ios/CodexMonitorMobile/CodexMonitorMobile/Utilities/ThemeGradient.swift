import SwiftUI

enum ThemeGradient: String, CaseIterable, Codable {
    // Light themes (optimized for glass effects)
    case skyBlue = "skyBlue"
    case cloudWhite = "cloudWhite"
    case icyBlue = "icyBlue"
    case softLavender = "softLavender"

    // Dark themes
    case midnightBlue = "midnightBlue"
    case oceanDeep = "oceanDeep"
    case cosmicPurple = "cosmicPurple"
    case slateDark = "slateDark"

    var displayName: String {
        switch self {
        // Light themes
        case .skyBlue: return "Sky Blue"
        case .cloudWhite: return "Cloud White"
        case .icyBlue: return "Icy Blue"
        case .softLavender: return "Soft Lavender"
        // Dark themes
        case .midnightBlue: return "Midnight Blue"
        case .oceanDeep: return "Ocean Deep"
        case .cosmicPurple: return "Cosmic Purple"
        case .slateDark: return "Slate Dark"
        }
    }

    var gradient: LinearGradient {
        switch self {
        // Light themes (optimized for glass effects)
        case .skyBlue:
            return LinearGradient(
                colors: [Color(hex: "E8F4FD"), Color(hex: "D4ECFC"), Color(hex: "F0F8FF")],
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
        case .cloudWhite:
            return LinearGradient(
                colors: [Color(hex: "F5F9FC"), Color(hex: "EDF4F9"), Color(hex: "FFFFFF")],
                startPoint: .top,
                endPoint: .bottom
            )
        case .icyBlue:
            return LinearGradient(
                colors: [Color(hex: "E0F0FF"), Color(hex: "CCE5FF"), Color(hex: "B8DAFF")],
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
        case .softLavender:
            return LinearGradient(
                colors: [Color(hex: "F0E6FF"), Color(hex: "E8DFFF"), Color(hex: "F5F0FF")],
                startPoint: .top,
                endPoint: .bottom
            )
        // Dark themes
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
        // Light themes
        case .skyBlue: return [Color(hex: "E8F4FD"), Color(hex: "D4ECFC")]
        case .cloudWhite: return [Color(hex: "F5F9FC"), Color(hex: "EDF4F9")]
        case .icyBlue: return [Color(hex: "E0F0FF"), Color(hex: "B8DAFF")]
        case .softLavender: return [Color(hex: "F0E6FF"), Color(hex: "E8DFFF")]
        // Dark themes
        case .midnightBlue: return [Color(hex: "0f0c29"), Color(hex: "302b63")]
        case .oceanDeep: return [Color(hex: "0d1b2a"), Color(hex: "415a77")]
        case .cosmicPurple: return [Color(hex: "10002b"), Color(hex: "3c096c")]
        case .slateDark: return [Color(hex: "1a1a1a"), Color(hex: "2d2d2d")]
        }
    }

    /// Whether this is a light theme (for determining text colors, etc.)
    var isLightTheme: Bool {
        switch self {
        case .skyBlue, .cloudWhite, .icyBlue, .softLavender:
            return true
        case .midnightBlue, .oceanDeep, .cosmicPurple, .slateDark:
            return false
        }
    }
}
