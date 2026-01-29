import SwiftUI

struct ProgressBarView: View {
    let progress: Double
    let color: Color
    var height: CGFloat = 8

    var body: some View {
        GeometryReader { proxy in
            let width = proxy.size.width
            ZStack(alignment: .leading) {
                Capsule()
                    .fill(Color.white.opacity(0.08))
                    .frame(height: height)
                Capsule()
                    .fill(color)
                    .frame(width: max(0, min(1, progress)) * width, height: height)
            }
        }
        .frame(height: height)
    }
}
