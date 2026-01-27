import SwiftUI
import CodexMonitorModels

struct SkillsView: View {
    @EnvironmentObject private var store: CodexStore
    @State private var validations: [SkillValidationResult] = []
    @State private var skills: [SkillOption] = []
    @State private var enabledSkills: Set<String> = []
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

                    GlassSectionHeader(title: "Enable / Disable", icon: "sparkles")
                    if skills.isEmpty {
                        Text("No skills loaded.")
                            .foregroundStyle(.secondary)
                    } else {
                        ForEach(skills) { skill in
                            Toggle(isOn: Binding(
                                get: { enabledSkills.contains(skill.id) },
                                set: { value in
                                    if value {
                                        enabledSkills.insert(skill.id)
                                    } else {
                                        enabledSkills.remove(skill.id)
                                    }
                                    Task { await persistSkillConfig() }
                                }
                            )) {
                                VStack(alignment: .leading) {
                                    Text(skill.name).font(.headline)
                                    if let desc = skill.description {
                                        Text(desc).font(.caption).foregroundStyle(.secondary)
                                    }
                                }
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
        skills = await store.skillsList(workspaceId: workspaceId)
        if let config = await store.skillsConfigRead(workspaceId: workspaceId) {
            enabledSkills = resolveEnabledSkills(skills: skills, config: config)
        } else {
            enabledSkills = Set(skills.map { $0.id })
        }
    }

    private func persistSkillConfig() async {
        guard let workspaceId = store.activeWorkspaceId else { return }
        let enabled = skills.filter { enabledSkills.contains($0.id) }
        let disabled = skills.filter { !enabledSkills.contains($0.id) }
        await store.skillsConfigWrite(workspaceId: workspaceId, enabled: enabled, disabled: disabled)
    }

    private func resolveEnabledSkills(skills: [SkillOption], config: JSONValue) -> Set<String> {
        let enabledEntries = config["enabled"]?.arrayValue ?? []
        let disabledEntries = config["disabled"]?.arrayValue ?? []

        let enabledKeys: Set<String> = Set(enabledEntries.compactMap { entry in
            let name = entry["name"]?.asString() ?? ""
            let path = entry["path"]?.asString() ?? ""
            if name.isEmpty || path.isEmpty { return nil }
            return "\(name)|\(path)"
        })

        let disabledKeys: Set<String> = Set(disabledEntries.compactMap { entry in
            let name = entry["name"]?.asString() ?? ""
            let path = entry["path"]?.asString() ?? ""
            if name.isEmpty || path.isEmpty { return nil }
            return "\(name)|\(path)"
        })

        if !enabledKeys.isEmpty {
            return enabledKeys
        }

        if !disabledKeys.isEmpty {
            let allKeys = Set(skills.map { $0.id })
            return allKeys.subtracting(disabledKeys)
        }

        return Set(skills.map { $0.id })
    }
}
