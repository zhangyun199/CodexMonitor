import SwiftUI

struct GradientBackground: View {
    @AppStorage("themeGradient") private var themeGradient = ThemeGradient.midnightBlue

    var body: some View {
        themeGradient.gradient
            .ignoresSafeArea()
    }
}

extension View {
    func withGradientBackground() -> some View {
        ZStack {
            GradientBackground()
            self
        }
    }
}
