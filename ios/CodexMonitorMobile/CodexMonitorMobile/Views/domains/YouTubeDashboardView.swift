import SwiftUI
import CodexMonitorModels

struct YouTubeDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var timeRange: LifeTimeRange

    private let stages: [PipelineStage] = [.brainDump, .development, .outline, .evaluation, .script, .edit, .published]

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                HStack {
                    Text("🎥 YouTube Pipeline")
                        .font(.headline)
                    Spacer()
                }

                TimeRangePicker(selection: $timeRange)

                if store.dashboardLoading {
                    ProgressView("Loading…")
                }

                if let error = store.dashboardError {
                    Text(error)
                        .foregroundStyle(.red)
                        .font(.caption)
                }

                LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 12) {
                    ForEach(stages, id: \.self) { stage in
                        StatCardView(title: stageLabel(stage), value: "\(store.youtubeDashboard?.pipelineStats[stage.rawValue] ?? 0)")
                    }
                }

                if let sTier = store.youtubeDashboard?.sTier, !sTier.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("S-Tier Ideas")
                            .font(.headline)
                        ForEach(sTier) { idea in
                            VStack(alignment: .leading, spacing: 4) {
                                Text(idea.title)
                                    .font(.subheadline)
                                Text("\(stageLabel(idea.stage)) • Tier \(idea.tier.rawValue)")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                            Divider()
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                }

                if let inProgress = store.youtubeDashboard?.inProgress, !inProgress.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("In Progress")
                            .font(.headline)
                        ForEach(inProgress) { idea in
                            VStack(alignment: .leading, spacing: 4) {
                                Text(idea.title)
                                    .font(.subheadline)
                                Text("\(stageLabel(idea.stage)) • Tier \(idea.tier.rawValue)")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                            Divider()
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                }
            }
            .padding()
        }
        .task(id: timeRange) {
            await store.fetchYouTubeDashboard(range: timeRange)
        }
    }

    private func stageLabel(_ stage: PipelineStage) -> String {
        switch stage {
        case .brainDump: return "Brain Dump"
        case .development: return "Development"
        case .outline: return "Outline"
        case .evaluation: return "Evaluation"
        case .script: return "Script"
        case .edit: return "Edit"
        case .published: return "Published"
        }
    }
}
