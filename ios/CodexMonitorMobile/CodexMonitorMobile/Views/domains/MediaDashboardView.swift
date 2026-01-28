import SwiftUI
import CodexMonitorModels

struct MediaDashboardView: View {
    @EnvironmentObject private var store: CodexStore

    @State private var selectedType: MediaType? = nil
    @State private var selectedStatus: MediaStatus? = nil
    @State private var searchText: String = ""
    @State private var sortOption: MediaSortOption = .rating
    @State private var viewMode: MediaViewMode = .grid

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

                ForEach(MediaType.allCases, id: \.self) { type in
                    let items = groupedItems[type] ?? []
                    if !items.isEmpty {
                        MediaSectionView(type: type, items: items, viewMode: viewMode)
                    }
                }
            }
            .padding()
        }
        .task {
            await store.fetchMediaLibrary()
        }
    }

    private var header: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text("🎬 Media Library")
                .font(.headline)
            if let library = store.mediaLibrary {
                Text("\(library.totalCount) items • \(library.completedCount) completed • ⭐ \(library.avgRating, specifier: "%.1f") avg")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }

    private var filterBar: some View {
        VStack(alignment: .leading, spacing: 8) {
            Picker("Type", selection: $selectedType) {
                Text("All").tag(MediaType?.none)
                ForEach(MediaType.allCases, id: \.self) { type in
                    Text(type.rawValue).tag(Optional(type))
                }
            }
            .pickerStyle(.segmented)

            Picker("Status", selection: $selectedStatus) {
                Text("All").tag(MediaStatus?.none)
                Text("Completed").tag(Optional(MediaStatus.completed))
                Text("Backlog").tag(Optional(MediaStatus.backlog))
            }
            .pickerStyle(.segmented)

            HStack {
                Picker("Sort", selection: $sortOption) {
                    ForEach(MediaSortOption.allCases, id: \.self) { option in
                        Text(option.label).tag(option)
                    }
                }
                .pickerStyle(.menu)

                Picker("View", selection: $viewMode) {
                    ForEach(MediaViewMode.allCases, id: \.self) { mode in
                        Text(mode.label).tag(mode)
                    }
                }
                .pickerStyle(.segmented)
            }

            TextField("Search titles…", text: $searchText)
                .textFieldStyle(.roundedBorder)
        }
    }

    private var filteredItems: [MediaItem] {
        guard let items = store.mediaLibrary?.items else { return [] }
        return items.filter { item in
            if let selectedType, item.mediaType != selectedType { return false }
            if let selectedStatus, item.status != selectedStatus { return false }
            if !searchText.isEmpty && !item.title.localizedCaseInsensitiveContains(searchText) {
                return false
            }
            return true
        }
    }

    private var groupedItems: [MediaType: [MediaItem]] {
        var groups: [MediaType: [MediaItem]] = [:]
        for item in filteredItems.sorted(by: sortComparator) {
            groups[item.mediaType, default: []].append(item)
        }
        return groups
    }

    private func sortComparator(_ lhs: MediaItem, _ rhs: MediaItem) -> Bool {
        switch sortOption {
        case .rating:
            return (lhs.rating ?? 0) > (rhs.rating ?? 0)
        case .title:
            return lhs.title < rhs.title
        case .updated:
            return lhs.updatedAt > rhs.updatedAt
        case .type:
            return lhs.mediaType.rawValue < rhs.mediaType.rawValue
        }
    }
}

private enum MediaSortOption: String, CaseIterable {
    case rating
    case title
    case updated
    case type

    var label: String {
        switch self {
        case .rating: return "Rating"
        case .title: return "Title"
        case .updated: return "Updated"
        case .type: return "Type"
        }
    }
}

private enum MediaViewMode: String, CaseIterable {
    case grid
    case list

    var label: String {
        switch self {
        case .grid: return "Grid"
        case .list: return "List"
        }
    }
}

private struct MediaSectionView: View {
    let type: MediaType
    let items: [MediaItem]
    let viewMode: MediaViewMode

    private let gridColumns = [GridItem(.flexible()), GridItem(.flexible())]

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("\(emoji) \(type.rawValue) (\(items.count))")
                .font(.headline)

            if viewMode == .grid {
                LazyVGrid(columns: gridColumns, spacing: 12) {
                    ForEach(items) { item in
                        MediaCardView(item: item)
                    }
                }
            } else {
                VStack(spacing: 8) {
                    ForEach(items) { item in
                        MediaCardView(item: item)
                    }
                }
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }

    private var emoji: String {
        switch type {
        case .film: return "🎬"
        case .tv: return "📺"
        case .anime: return "🎌"
        case .game: return "🎮"
        case .book: return "📚"
        case .youTube: return "🎥"
        }
    }
}

private struct MediaCardView: View {
    let item: MediaItem

    var body: some View {
        ZStack(alignment: .bottomLeading) {
            RoundedRectangle(cornerRadius: 10)
                .fill(gradient)
                .frame(height: 180)
                .opacity(item.status == .backlog ? 0.6 : 1)

            VStack(alignment: .leading, spacing: 4) {
                Text(item.title)
                    .font(.caption)
                    .bold()
                    .lineLimit(2)
                    .foregroundStyle(.white)
                Text(item.rating != nil ? "⭐ \(item.rating!, specifier: "%.1f")" : "—")
                    .font(.caption2)
                    .foregroundStyle(.white.opacity(0.8))
            }
            .padding(8)
        }
    }

    private var gradient: LinearGradient {
        switch item.mediaType {
        case .film:
            return LinearGradient(colors: [.blue, .blue.opacity(0.6)], startPoint: .top, endPoint: .bottom)
        case .tv:
            return LinearGradient(colors: [.purple, .purple.opacity(0.6)], startPoint: .top, endPoint: .bottom)
        case .anime:
            return LinearGradient(colors: [.pink, .pink.opacity(0.6)], startPoint: .top, endPoint: .bottom)
        case .game:
            return LinearGradient(colors: [.green, .green.opacity(0.6)], startPoint: .top, endPoint: .bottom)
        case .book:
            return LinearGradient(colors: [.orange, .orange.opacity(0.6)], startPoint: .top, endPoint: .bottom)
        case .youTube:
            return LinearGradient(colors: [.red, .red.opacity(0.6)], startPoint: .top, endPoint: .bottom)
        }
    }
}
