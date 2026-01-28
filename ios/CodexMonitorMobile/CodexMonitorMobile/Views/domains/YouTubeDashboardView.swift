import SwiftUI
import CodexMonitorModels

struct YouTubeDashboardView: View {
    @EnvironmentObject private var store: CodexStore

    @State private var selectedTier: YouTubeTier? = nil
    @State private var selectedStage: YouTubeStage? = nil
    @State private var searchText: String = ""
    @State private var sortOption: YouTubeSortOption = .tier
    @State private var viewMode: YouTubeViewMode = .grid

    private let columns = [GridItem(.flexible()), GridItem(.flexible())]

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                header
                filterBar

                if store.dashboardLoading {
                    ProgressView("Loading…")
                }

                if let error = store.dashboardError {
                    Text(error)
                        .foregroundStyle(.red)
                        .font(.caption)
                }

                ForEach(YouTubeTier.allCases, id: \.self) { tier in
                    let ideas = groupedIdeas[tier] ?? []
                    if !ideas.isEmpty {
                        YouTubeSectionView(tier: tier, ideas: ideas, viewMode: viewMode)
                    }
                }
            }
            .padding()
        }
        .task {
            await store.fetchYouTubeLibrary()
        }
    }

    private var header: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text("🎥 YouTube Ideas")
                .font(.headline)
            if let library = store.youtubeLibrary {
                Text("\(library.totalCount) ideas • \(library.inProgressCount) in progress • \(library.publishedCount) published")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }

    private var filterBar: some View {
        VStack(alignment: .leading, spacing: 8) {
            Picker("Tier", selection: $selectedTier) {
                Text("All").tag(YouTubeTier?.none)
                ForEach(YouTubeTier.allCases, id: \.self) { tier in
                    Text(tier.rawValue).tag(Optional(tier))
                }
            }
            .pickerStyle(.segmented)

            Picker("Stage", selection: $selectedStage) {
                Text("All").tag(YouTubeStage?.none)
                ForEach(YouTubeStage.allCases, id: \.self) { stage in
                    Text(stage.rawValue).tag(Optional(stage))
                }
            }
            .pickerStyle(.segmented)

            HStack {
                Picker("Sort", selection: $sortOption) {
                    ForEach(YouTubeSortOption.allCases, id: \.self) { option in
                        Text(option.label).tag(option)
                    }
                }
                .pickerStyle(.menu)

                Picker("View", selection: $viewMode) {
                    ForEach(YouTubeViewMode.allCases, id: \.self) { mode in
                        Text(mode.label).tag(mode)
                    }
                }
                .pickerStyle(.segmented)
            }

            TextField("Search ideas…", text: $searchText)
                .textFieldStyle(.roundedBorder)
        }
    }

    private var filteredIdeas: [YouTubeIdea] {
        guard let ideas = store.youtubeLibrary?.items else { return [] }
        return ideas.filter { idea in
            if let selectedTier, idea.tier != selectedTier { return false }
            if let selectedStage, idea.stage != selectedStage { return false }
            if !searchText.isEmpty && !idea.title.localizedCaseInsensitiveContains(searchText) {
                return false
            }
            return true
        }
    }

    private var groupedIdeas: [YouTubeTier: [YouTubeIdea]] {
        var groups: [YouTubeTier: [YouTubeIdea]] = [:]
        for idea in filteredIdeas.sorted(by: sortComparator) {
            groups[idea.tier, default: []].append(idea)
        }
        return groups
    }

    private func sortComparator(_ lhs: YouTubeIdea, _ rhs: YouTubeIdea) -> Bool {
        switch sortOption {
        case .title:
            return lhs.title < rhs.title
        case .stage:
            return lhs.stage.rawValue < rhs.stage.rawValue
        case .updated:
            return lhs.updatedAt > rhs.updatedAt
        case .tier:
            return lhs.tier.rawValue < rhs.tier.rawValue
        }
    }
}

private enum YouTubeSortOption: String, CaseIterable {
    case tier
    case stage
    case title
    case updated

    var label: String {
        switch self {
        case .tier: return "Tier"
        case .stage: return "Stage"
        case .title: return "Title"
        case .updated: return "Updated"
        }
    }
}

private enum YouTubeViewMode: String, CaseIterable {
    case grid
    case list

    var label: String {
        switch self {
        case .grid: return "Grid"
        case .list: return "List"
        }
    }
}

private struct YouTubeSectionView: View {
    let tier: YouTubeTier
    let ideas: [YouTubeIdea]
    let viewMode: YouTubeViewMode

    private let gridColumns = [GridItem(.flexible()), GridItem(.flexible())]

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("\(label) (\(ideas.count))")
                .font(.headline)

            if viewMode == .grid {
                LazyVGrid(columns: gridColumns, spacing: 12) {
                    ForEach(ideas) { idea in
                        YouTubeCardView(idea: idea)
                    }
                }
            } else {
                VStack(spacing: 8) {
                    ForEach(ideas) { idea in
                        YouTubeCardView(idea: idea)
                    }
                }
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }

    private var label: String {
        switch tier {
        case .s: return "⭐ S-Tier"
        case .a: return "🥈 A-Tier"
        case .b: return "🥉 B-Tier"
        case .c: return "🪨 C-Tier"
        }
    }
}

private struct YouTubeCardView: View {
    let idea: YouTubeIdea

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            RoundedRectangle(cornerRadius: 8)
                .fill(gradient)
                .overlay(alignment: .topLeading) { tierBadge }
                .overlay(alignment: .topTrailing) { stageBadge }
                .aspectRatio(2 / 3, contentMode: .fit)

            Text(idea.title)
                .font(.caption)
                .fontWeight(.semibold)
                .lineLimit(1)
        }
    }

    private var tierBadge: some View {
        Text(idea.tier.rawValue)
            .font(.caption2)
            .fontWeight(.bold)
            .padding(.vertical, 2)
            .padding(.horizontal, 6)
            .background(tierColor.opacity(0.9))
            .foregroundStyle(tierTextColor)
            .clipShape(RoundedRectangle(cornerRadius: 4))
            .padding(6)
    }

    private var stageBadge: some View {
        Text(stageLabel)
            .font(.caption2)
            .fontWeight(.bold)
            .padding(.vertical, 2)
            .padding(.horizontal, 6)
            .background(Color.black.opacity(0.75))
            .foregroundStyle(.white)
            .clipShape(RoundedRectangle(cornerRadius: 4))
            .padding(6)
    }

    private var stageLabel: String {
        idea.stage.rawValue.capitalized
    }

    private var tierColor: Color {
        switch idea.tier {
        case .s:
            return .yellow
        case .a:
            return .gray
        case .b:
            return .orange
        case .c:
            return .gray.opacity(0.7)
        }
    }

    private var tierTextColor: Color {
        switch idea.tier {
        case .c:
            return .white
        default:
            return .black
        }
    }

    private var gradient: LinearGradient {
        switch idea.tier {
        case .s:
            return LinearGradient(colors: [.yellow, .orange], startPoint: .top, endPoint: .bottom)
        case .a:
            return LinearGradient(colors: [.gray, .gray.opacity(0.7)], startPoint: .top, endPoint: .bottom)
        case .b:
            return LinearGradient(colors: [.orange, .brown], startPoint: .top, endPoint: .bottom)
        case .c:
            return LinearGradient(colors: [.gray.opacity(0.7), .gray], startPoint: .top, endPoint: .bottom)
        }
    }
}
