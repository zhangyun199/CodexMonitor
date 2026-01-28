import SwiftUI
import CodexMonitorModels

struct DomainTabBar: View {
    @Binding var selection: LifeDomain?

    private let columns = [GridItem(.flexible()), GridItem(.flexible()), GridItem(.flexible())]

    var body: some View {
        LazyVGrid(columns: columns, spacing: 8) {
            Button {
                selection = nil
            } label: {
                HStack(spacing: 6) {
                    Image(systemName: "bubble.left.and.text.bubble.right")
                    Text("Chat")
                }
                .frame(maxWidth: .infinity)
            }
            .buttonStyle(.bordered)
            .tint(selection == nil ? .blue : .gray)

            ForEach(LifeDomain.allCases) { domain in
                Button {
                    selection = domain
                } label: {
                    Text(domain.rawValue.capitalized)
                        .frame(maxWidth: .infinity)
                }
                .buttonStyle(.bordered)
                .tint(selection == domain ? .blue : .gray)
            }
        }
        .padding(.horizontal)
        .padding(.top, 8)
    }
}
