import Foundation
#if canImport(UIKit)
import UIKit
#endif

public enum CodexMonitorRendering {
    public static func markdown(_ text: String) -> AttributedString {
        if let attributed = try? AttributedString(markdown: text) {
            return attributed
        }
        return AttributedString(text)
    }

    public static func monospaced(_ text: String) -> AttributedString {
        var attributed = AttributedString(text)
        #if canImport(UIKit)
        attributed.font = UIFont.monospacedSystemFont(ofSize: 14, weight: .regular)
        #endif
        return attributed
    }
}
