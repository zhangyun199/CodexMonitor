import SwiftUI
import CodexMonitorModels

struct SkillsView: View {
    @EnvironmentObject private var store: CodexStore
    @State private var validations: [SkillValidationResult] = []
    @State private var installUrl: String = ""

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 16) {
                    GlassSectionHeader(title: "Validation", icon: "checkmark.seal")
                    Button("Refresh") { Task { await refresh() } }

                    if validations.isEmpty {
                        Text("No skills found or no validation issues.")
                            .foregroundStyle(.secondary)
                    } else {
                        ForEach(validations) { result in
                            VStack(alignment: .leading, spacing: 8) {
                                Text(result.name).font(.headline)
                                if let desc = result.description {
                                    Text(desc).font(.subheadline).foregroundStyle(.secondary)
                                }
                                if !result.issues.isEmpty {
                                    ForEach(result.issues, id: \.self) { issue in
                                        Text("⚠️ \(issue)")
                                            .font(.footnote)
                                            .foregroundStyle(.orange)
                                    }
                                }
                            }
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .padding()
                            .background(.ultraThinMaterial)
                            .cornerRadius(12)
                        }
                    }

                    GlassSectionHeader(title: "Install", icon: "tray.and.arrow.down")
                    HStack {
                        TextField("Git URL", text: $installUrl)
                            .textInputAutocapitalization(.never)
                        Button("Install") {
                            guard let workspaceId = store.activeWorkspaceId else { return }
                            Task {
                                await store.skillsInstallFromGit(sourceUrl: installUrl, target: "workspace", workspaceId: workspaceId)
                                installUrl = ""
                                await refresh()
                            }
                        }
                    }
                }
                .padding()
            }
            .navigationTitle("Skills")
        }
        .task { await refresh() }
    }

    private func refresh() async {
        guard let workspaceId = store.activeWorkspaceId else { return }
        validations = await store.skillsValidate(workspaceId: workspaceId)
    }
}
