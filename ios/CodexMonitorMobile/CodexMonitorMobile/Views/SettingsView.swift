import SwiftUI

struct SettingsView: View {
    @EnvironmentObject private var store: CodexStore
    @Environment(\.dismiss) private var dismiss
    @AppStorage("themeGradient") private var themeGradient = ThemeGradient.midnightBlue
    @State private var testResult: String?
    @State private var isTesting = false

    var body: some View {
        NavigationStack {
            Form {
                Section("Appearance") {
                    Picker("Background", selection: $themeGradient) {
                        ForEach(ThemeGradient.allCases, id: \.self) { gradient in
                            HStack {
                                // Preview swatch
                                LinearGradient(
                                    colors: gradient.previewColors,
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                                .frame(width: 30, height: 20)
                                .clipShape(RoundedRectangle(cornerRadius: 4))

                                Text(gradient.displayName)
                            }
                            .tag(gradient)
                        }
                    }
                }

                Section("Connection") {
                    TextField("Host", text: $store.host)
                        .textInputAutocapitalization(.never)
                        .autocorrectionDisabled()
                    TextField("Port", text: $store.port)
                        .keyboardType(.numberPad)
                    SecureField("Token", text: $store.token)
                }
                Section {
                    Button("Test Connection") {
                        testConnection()
                    }
                    if let testResult {
                        Text(testResult)
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                }
            }
            .navigationTitle("Settings")
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Close") { dismiss() }
                }
            }
        }
    }

    private func testConnection() {
        guard !isTesting else { return }
        isTesting = true
        testResult = nil
        store.saveSettings()
        Task {
            store.connect()
            try? await Task.sleep(nanoseconds: 400_000_000)
            let ok = await store.ping()
            testResult = ok ? "✅ Connected" : "⚠️ Ping failed"
            isTesting = false
        }
    }
}
