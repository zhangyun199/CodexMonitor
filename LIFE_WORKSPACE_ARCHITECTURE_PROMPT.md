# Life Workspace Architecture Design Prompt

**For:** ChatGPT Pro 5.2
**Project:** CodexMonitor - Life Workspace Redesign
**Date:** January 28, 2026

---

## Executive Summary

Design a comprehensive architecture for transforming CodexMonitor from multiple domain-specific workspaces into a **simplified two-workspace system**:

1. **CodexMonitor** — Coding workspace (unchanged)
2. **Life** — Single unified workspace for all life domains with domain-specific dashboard views

The Life workspace combines interactive AI chat (threads talking to Codex) with **rendered domain views** in the right panel that replace the center panel when selected (similar to how Git diff currently works).

---

## What You're Designing

### Deliverables Expected

1. **System Architecture Document** — Data flow, component hierarchy, state management
2. **Type Definitions** — TypeScript types for all new structures
3. **React Component Architecture** — Component tree, props, hooks
4. **Rust Backend Changes** — Daemon modifications, new Tauri commands
5. **iOS SwiftUI Architecture** — View hierarchy, CodexStore additions
6. **Supabase Schema** — Tables needed for aggregations
7. **Test Specifications** — What to test, edge cases
8. **Implementation Order** — Phased approach with dependencies

---

## Current State (Attached ZIP)

The attached CodexMonitor.zip contains the full codebase. Key paths:

| Component | Path |
|-----------|------|
| Rust Daemon | `src-tauri/src/bin/codex_monitor_daemon.rs` |
| Rust Types | `src-tauri/src/types.rs` |
| Domains Module | `src-tauri/src/domains.rs` |
| React Types | `src/types.ts` |
| Domain Components | `src/features/domains/` |
| Settings UI | `src/features/settings/components/SettingsView.tsx` |
| iOS Models | `ios/Packages/CodexMonitorRPC/Sources/CodexMonitorModels/Models.swift` |
| iOS Views | `ios/CodexMonitorMobile/CodexMonitorMobile/Views/` |
| iOS Store | `ios/CodexMonitorMobile/CodexMonitorMobile/CodexStore.swift` |

### What Already Exists

**Domains System (partially built):**
- Domain CRUD in Rust (`domains.rs`) ✅
- Domain types across platforms ✅
- Workspace→Domain assignment via `domainId` ✅
- Domain trends computation from Obsidian ✅
- DomainPanel component (shows trends) ✅
- Domain tab in right panel ✅

**What's Missing:**
- Thread-level system prompt injection ❌
- Domain views that REPLACE center panel ❌
- Combined multi-domain prompt injection ❌
- Domain-specific rendered dashboards ❌
- iOS domain management UI ❌

---

## Target Architecture

### Workspace Structure

```
CodexMonitor App
├── Workspaces (Left Sidebar)
│   ├── CodexMonitor (coding - unchanged)
│   └── Life (NEW - unified life workspace)
│
└── Life Workspace Layout
    ├── Center Panel
    │   ├── DEFAULT: Chat threads with Codex
    │   │   └── Full life context injected (all 4 domain prompts combined)
    │   │
    │   └── WHEN DOMAIN SELECTED: Domain Dashboard View
    │       └── Rendered React component (not markdown)
    │
    └── Right Panel
        ├── Domain Tabs: [Delivery] [Food] [Exercise] [Media] [YouTube] [Bills]
        └── Clicking domain → Center panel transforms to that domain's view
```

### Interaction Flow

```
User in Life Workspace
    │
    ├─→ Default: Chat with Codex (full life context)
    │   └── "Had eggs for breakfast" → Codex responds with nutrition table
    │
    └─→ Clicks "Delivery" in right panel
        └── Center panel REPLACES with DeliveryDashboardView
            ├── Stats cards (earnings, orders, hourly rate)
            ├── Today's orders table
            ├── Time filter: [Today] [Week] [Month] [Lifetime]
            └── Click "Back to Chat" or another domain to switch
```

### Domain Views (6 domains)

Each domain needs a custom React component that renders when selected:

| Domain | Key Features |
|--------|--------------|
| **Delivery** | Earnings stats, order table, $/hr, $/mi, merchant analysis |
| **Food/Nutrition** | Daily macros, meal log, weekly trends, macro breakdown charts |
| **Exercise** | Workout log, streak tracking, weekly progress grid |
| **Media** | Bookshelf-style covers, ratings, type filters, recently watched |
| **YouTube** | Pipeline dashboard (S/A/B/C tiers), video ideas, stage tracking |
| **Bills/Finance** | Due dates, monthly totals, payment history, calendar view |

---

## Design Philosophy

### From User's Personal Logging Style (PDF Examples)

The user has strong visual preferences from years of iPad logging:

**Color Semantics:**
| Color | Meaning |
|-------|---------|
| Yellow |  headers |
| Cyan/Teal | Categories, media titles, labels |
| Green | Rewards, effects, positive outcomes |
| Red | Warnings, costs, negative feelings |
| White | General notes, descriptions |
| Grey | Dates, times

**Core Principles:**
1. **Dark mode with grid** — Graph paper aesthetic, subtle grid background
2. **Images are FIRST CLASS** — Media covers, food photos, merchant badges inline
3. **Cause → Effect thinking** — Action → Reward visual patterns
4. **Timestamps matter** — Every entry has specific time
5. **Compact but rich** — Lots of info in small space, not sparse
6. **Not overwhelming** — Smart progressive disclosure (Schwartz's paradox of choice)
7. **Mobile-first** — Glanceable from iPad while driving

### From life-radar Project (Reference Implementation)

The user has an existing life-radar dashboard (in `code/_archive/life-radar/`) with patterns to follow:

**Component Patterns:**
```typescript
// Hook-based data fetching
const { items, stats, loading, error, refetch } = useMedia(typeFilter);

// Loading skeleton
{loading && <div className="animate-pulse">...</div>}

// Type-safe filtering
type MediaType = 'film' | 'tv' | 'game' | 'book' | 'anime';
```

**Styling Conventions:**
- Tailwind CSS with semantic color names
- `bg-gray-800/50` for card backgrounds
- `border-gray-700` for borders
- Domain-specific accent colors:
  - Delivery: `blue-500`
  - Nutrition: `green-500`
  - Finance: `yellow-500`
  - YouTube: `red-500`
  - Media: `purple-500`

**Layout Patterns:**
- Stats grid: 4 cards in a row
- List with hover states
- Type filter pills (horizontal)
- Time range selector

---

## Data Architecture

### Data Sources

| Need | Source | Why |
|------|--------|-----|
| Today's entries | Obsidian Stream | Fast, local, real-time |
| Week/month/lifetime aggregations | Supabase | Pre-computed, performant |
| Entity details | Obsidian Entities folder | Source of truth |
| Images/covers | External APIs (TMDB, IGDB) or stored URLs | Visual richness |

### Obsidian Vault Structure

```
/Volumes/YouTube 4TB/Obsidian/
├── Stream/
│   └── 2026-01.md              ← Monthly logs (primary input)
├── Entities/
│   ├── Delivery/
│   │   └── Sessions/           ← 29 shift logs with YAML
│   ├── Finance/
│   │   └── Bills/              ← 20 bill entity files
│   ├── Food/                   ← 26 food entities
│   ├── Media/                  ← 173 media entities
│   ├── YouTube/                ← 210 video ideas
│   └── Behaviors/              ← Exercise behaviors
├── Domains/                    ← Dashboard pages
├── Indexes/                    ← JSON aggregation files
└── _config/                    ← YAML configs
```

### Supabase Tables Needed

```sql
-- Aggregated delivery stats (computed daily)
CREATE TABLE delivery_aggregations (
  id UUID PRIMARY KEY,
  period TEXT NOT NULL,  -- 'day', 'week', 'month', 'lifetime'
  period_start DATE NOT NULL,
  period_end DATE NOT NULL,
  total_earnings DECIMAL,
  order_count INT,
  total_hours DECIMAL,
  hourly_rate DECIMAL,
  per_mile_rate DECIMAL,
  computed_at TIMESTAMPTZ DEFAULT NOW()
);

-- Aggregated nutrition stats
CREATE TABLE nutrition_aggregations (
  id UUID PRIMARY KEY,
  period TEXT NOT NULL,
  period_start DATE NOT NULL,
  period_end DATE NOT NULL,
  avg_calories DECIMAL,
  avg_protein DECIMAL,
  avg_carbs DECIMAL,
  avg_fat DECIMAL,
  avg_fiber DECIMAL,
  meals_logged INT,
  computed_at TIMESTAMPTZ DEFAULT NOW()
);

-- Similar for: exercise_aggregations, media_aggregations, youtube_aggregations, finance_aggregations
```

---

## Life Workspace Prompt Injection

When a thread starts in the Life workspace, inject ALL domain prompts combined. This gives Codex full life context regardless of what the user is talking about.

### Combined System Prompt Structure

```
[workspace-delivery-finance.md content]
---
[workspace-food-exercise.md content]
---
[workspace-media.md content]
---
[workspace-youtube.md content]
---

You are JMWillis's life assistant with full context across all domains.
Auto-detect the domain from user messages and respond appropriately.
```

### Domain Detection Logic

When user sends a message, detect domain by keywords:

| Domain | Trigger Keywords |
|--------|------------------|
| Delivery | order, miles, doordash, uber, grubhub, merchant names, $/mi |
| Nutrition | ate, had, breakfast, lunch, dinner, calories, protein, macros |
| Exercise | workout, walk, gym, strength, run, exercise |
| Media | watched, finished, playing, rating, movie, show, anime, game |
| YouTube | video idea, pipeline, thesis, hook, script, tier |
| Finance | bill, paid, due, spent, expense |

---

## Domain Prompt Files (Full Content)

Include these in the Life workspace's combined system prompt.

### workspace-delivery-finance.md

```markdown
[FULL CONTENT OF THE FILE - approximately 816 lines]
```

**Key sections:**
- Driver profile (home base, vehicle, schedule, targets)
- Merchant tiers (S/A/B/C/D with wait times, hourly rates)
- STT parsing rules (speech-to-text aliases)
- Order evaluation logic ($/mi thresholds, AR zones)
- Intersection distances (deadhead calculations)
- Response formats (order advice, session complete)
- Finance: Monthly bills schedule ($2,653 total)

### workspace-food-exercise.md

```markdown
[FULL CONTENT OF THE FILE - approximately 1098 lines]
```

**Key sections:**
- User profile (37yo, 235lbs → 180-185lbs goal)
- Genetic profile (FTO, FADS1, MTNR1B, MCM6, etc.)
- Daily nutrition targets (2,100-2,300 cal, 120-170g protein)
- Food entity file formats
- Behavior tracking (Morning Walk, Strength Training)
- Stream logging format

### workspace-media.md

```markdown
[FULL CONTENT OF THE FILE - approximately 679 lines]
```

**Key sections:**
- Rating scale (1-10, with 7 as the threshold)
- Taste profile (dealbreakers, medium-specific knowledge)
- Media entity file format
- Recommendation engine structure
- Library stats (172 items, 33 perfect 10s)

### workspace-youtube.md

```markdown
[FULL CONTENT OF THE FILE - approximately 693 lines]
```

**Key sections:**
- Tier system (S/A/B/C)
- Pipeline stages (brain_dump → published)
- Video structure template (Hook, Frame, Thesis, Pillars, Conclusion)
- Idea fields (title, thesis, pillars, hooks, evidence, tier)
- Working modes (Brain Dump, Development, Outline, Evaluation, Pipeline)

---

## Technical Requirements

### Type Definitions

```typescript
// Domain types (extend existing)
type LifeDomain =
  | 'delivery'
  | 'nutrition'
  | 'exercise'
  | 'media'
  | 'youtube'
  | 'finance';

interface DomainViewState {
  activeDomain: LifeDomain | null;
  timeRange: 'today' | 'week' | 'month' | 'lifetime';
  filters: Record<string, string>;
  sortBy: string;
  sortDirection: 'asc' | 'desc';
}

// Domain dashboard data structures
interface DeliveryDashboard {
  stats: {
    totalEarnings: number;
    orderCount: number;
    activeHours: number;
    hourlyRate: number;
    perMileRate: number;
  };
  orders: DeliveryOrder[];
  topMerchants: MerchantStats[];
  timeRange: string;
}

interface NutritionDashboard {
  today: {
    calories: number;
    protein: number;
    carbs: number;
    fat: number;
    fiber: number;
    meals: MealEntry[];
  };
  targets: NutritionTargets;
  weeklyTrend: DailyNutrition[];
}

interface MediaDashboard {
  recentlyWatched: MediaItem[];
  stats: {
    backlogCount: number;
    completedCount: number;
    avgRating: number;
  };
  byType: Record<MediaType, number>;
}

interface YouTubeDashboard {
  pipelineStats: Record<PipelineStage, number>;
  sTierIdeas: VideoIdea[];
  inProgress: VideoIdea[];
  recentActivity: PipelineEvent[];
}

interface FinanceDashboard {
  upcomingBills: Bill[];
  monthlyTotal: number;
  paidThisMonth: number;
  remainingThisMonth: number;
  calendarView: BillCalendarDay[];
}
```

### React Component Architecture

```
src/features/life/
├── components/
│   ├── LifeWorkspaceView.tsx       # Main container
│   ├── DomainSelector.tsx          # Right panel domain tabs
│   ├── DomainViewContainer.tsx     # Switches between domain views
│   │
│   ├── domains/
│   │   ├── DeliveryDashboard.tsx
│   │   ├── NutritionDashboard.tsx
│   │   ├── ExerciseDashboard.tsx
│   │   ├── MediaDashboard.tsx
│   │   ├── YouTubeDashboard.tsx
│   │   └── FinanceDashboard.tsx
│   │
│   └── shared/
│       ├── StatCard.tsx
│       ├── TimeRangeSelector.tsx
│       ├── DataTable.tsx
│       ├── CoverImage.tsx
│       └── ProgressBar.tsx
│
├── hooks/
│   ├── useLifeWorkspace.ts         # State management
│   ├── useDomainDashboard.ts       # Generic data fetching
│   ├── useDeliveryData.ts
│   ├── useNutritionData.ts
│   ├── useMediaData.ts
│   ├── useYouTubeData.ts
│   └── useFinanceData.ts
│
└── styles/
    └── life-dashboard.css
```

### Rust Backend Additions

```rust
// New Tauri commands needed

#[tauri::command]
async fn get_life_workspace_prompt() -> Result<String, String> {
    // Combine all 4 domain prompts into one
    // Return for injection into thread/start
}

#[tauri::command]
async fn get_delivery_dashboard(
    workspace_id: String,
    range: String  // "today", "week", "month", "lifetime"
) -> Result<DeliveryDashboard, String> {
    // Query Obsidian sessions + Supabase aggregations
}

#[tauri::command]
async fn get_nutrition_dashboard(
    workspace_id: String,
    range: String
) -> Result<NutritionDashboard, String> {
    // Parse stream file + food entities + Supabase
}

// Similar for: get_media_dashboard, get_youtube_dashboard, get_finance_dashboard
```

### iOS Architecture

```swift
// New views needed
ios/CodexMonitorMobile/CodexMonitorMobile/Views/
├── LifeWorkspaceView.swift         # Main life workspace container
├── DomainTabBar.swift              # Domain selector tabs
├── domains/
│   ├── DeliveryDashboardView.swift
│   ├── NutritionDashboardView.swift
│   ├── ExerciseDashboardView.swift
│   ├── MediaDashboardView.swift
│   ├── YouTubeDashboardView.swift
│   └── FinanceDashboardView.swift
└── shared/
    ├── StatCardView.swift
    ├── CoverImageView.swift
    └── TimeRangePicker.swift

// CodexStore additions
extension CodexStore {
    func fetchDeliveryDashboard(range: String) async
    func fetchNutritionDashboard(range: String) async
    func fetchMediaDashboard(range: String) async
    func fetchYouTubeDashboard(range: String) async
    func fetchFinanceDashboard(range: String) async
}
```

---

## Progressive Disclosure Strategy

To avoid overwhelming the user with data (Schwartz's paradox of choice):

### Smart Defaults
- **Default time range:** Today (not lifetime)
- **Default sort:** Most recent first
- **Collapsed sections:** Expand on tap/click
- **"Show more" pattern:** Load 10-20 items, button to load more

### Domain-Specific Disclosure

| Domain | Initial View | Expanded View |
|--------|--------------|---------------|
| Delivery | Today's stats + last 5 orders | Full order history with filters |
| Nutrition | Today's macros + meals | Weekly trends, food frequency |
| Media | Last 10 watched + backlog | Full library with type filters |
| YouTube | S-tier + in-progress | Full pipeline by tier |
| Finance | Next 7 days bills | Full month calendar |

### Visual Hierarchy
1. **Hero stat** — The most important number, large and prominent
2. **Supporting stats** — 3-4 secondary stats in a row
3. **Recent activity** — Compact list, most recent first
4. **Deep dive** — Hidden until expanded or filtered

---

## Implementation Phases

### Phase 0: Foundation (No UI changes)
- [ ] Create Life workspace type in backend
- [ ] Implement combined prompt injection for Life workspace threads
- [ ] Add domain dashboard Tauri commands (empty implementations)
- [ ] Create Supabase aggregation tables

### Phase 1: Delivery Dashboard (Proof of concept)
- [ ] Build DeliveryDashboard React component
- [ ] Implement get_delivery_dashboard Rust command
- [ ] Wire up domain selection → center panel replacement
- [ ] Add time range selector
- [ ] iOS DeliveryDashboardView

### Phase 2: Second Domain (Learn patterns)
- [ ] Choose: Media or YouTube (both have visual richness)
- [ ] Build dashboard component following Phase 1 patterns
- [ ] Refactor shared components (StatCard, TimeRangeSelector)
- [ ] iOS equivalent

### Phase 3: Remaining Domains
- [ ] Nutrition dashboard
- [ ] Exercise dashboard
- [ ] Finance dashboard
- [ ] YouTube OR Media (whichever wasn't done in Phase 2)

### Phase 4: Polish
- [ ] Supabase aggregation cron jobs
- [ ] Caching layer for dashboard data
- [ ] Loading states and skeletons
- [ ] Error handling
- [ ] Filter persistence

---

## Test Specifications

### Unit Tests

```typescript
// Domain detection
describe('detectDomainFromMessage', () => {
  it('detects delivery from order keywords', () => {
    expect(detectDomain('doordash panera 12 3 miles')).toBe('delivery');
  });

  it('detects nutrition from food keywords', () => {
    expect(detectDomain('had eggs for breakfast')).toBe('nutrition');
  });

  it('returns null for ambiguous messages', () => {
    expect(detectDomain('hello')).toBeNull();
  });
});

// Dashboard data hooks
describe('useDeliveryData', () => {
  it('fetches today data by default', async () => {
    const { result } = renderHook(() => useDeliveryData());
    await waitFor(() => expect(result.current.loading).toBe(false));
    expect(result.current.stats.timeRange).toBe('today');
  });

  it('persists filter state', async () => {
    const { result, rerender } = renderHook(() => useDeliveryData());
    act(() => result.current.setTimeRange('week'));
    rerender();
    expect(result.current.stats.timeRange).toBe('week');
  });
});
```

### Integration Tests

```typescript
describe('Life Workspace', () => {
  it('injects combined prompt on thread start', async () => {
    const thread = await startThreadInWorkspace('life');
    expect(thread.systemPrompt).toContain('delivery');
    expect(thread.systemPrompt).toContain('nutrition');
    expect(thread.systemPrompt).toContain('media');
    expect(thread.systemPrompt).toContain('youtube');
  });

  it('switches center panel when domain selected', async () => {
    render(<LifeWorkspaceView />);
    await userEvent.click(screen.getByText('Delivery'));
    expect(screen.getByTestId('delivery-dashboard')).toBeInTheDocument();
    expect(screen.queryByTestId('chat-panel')).not.toBeInTheDocument();
  });

  it('returns to chat when back button clicked', async () => {
    render(<LifeWorkspaceView activeDomain="delivery" />);
    await userEvent.click(screen.getByText('Back to Chat'));
    expect(screen.getByTestId('chat-panel')).toBeInTheDocument();
  });
});
```

### E2E Tests

```typescript
describe('Life Workspace E2E', () => {
  it('logs meal and updates nutrition dashboard', async () => {
    // 1. Navigate to Life workspace
    // 2. Send "had eggs and toast for breakfast"
    // 3. Verify Codex responds with nutrition breakdown
    // 4. Click Nutrition in right panel
    // 5. Verify today's macros include the meal
  });

  it('shows delivery stats from Obsidian sessions', async () => {
    // 1. Navigate to Life workspace
    // 2. Click Delivery in right panel
    // 3. Verify stats match Obsidian session data
    // 4. Change time range to "week"
    // 5. Verify aggregated stats update
  });
});
```

---

## Questions for Clarification

Before finalizing the architecture, consider these decisions:

1. **Domain view navigation:** Should domains be tabs (switch instantly) or routes (browser back button works)?

2. **Hybrid data strategy:** For week/month/lifetime, should we:
   - Compute on-demand from Obsidian (slow but always fresh)?
   - Pre-compute to Supabase on a schedule (fast but potentially stale)?
   - Hybrid (cache with TTL, recompute on demand if stale)?

3. **Image handling:** For media covers and food photos:
   - Fetch from external APIs (TMDB, IGDB) on demand?
   - Cache locally after first fetch?
   - Store URLs in entity files?

4. **Filter persistence:** Where should filter state live?
   - Local React state (resets on navigation)?
   - Context/Zustand (persists within session)?
   - LocalStorage (persists across sessions)?

5. **iOS parity:** Should iOS domain views be:
   - Native SwiftUI (full parity, more work)?
   - WebView of React components (less work, less native feel)?
   - Simplified native views (80% parity)?

---

## Output Format

Please provide your architecture design in the following structure:

1. **Executive Summary** — One-page overview
2. **System Architecture** — Component diagram, data flow diagram
3. **Type Definitions** — Complete TypeScript types
4. **Component Specifications** — For each React component
5. **Backend Specifications** — Rust commands, Supabase schema
6. **iOS Specifications** — SwiftUI views, CodexStore methods
7. **Implementation Plan** — Phased approach with task breakdown
8. **Test Plan** — What to test at each phase
9. **Risk Assessment** — What could go wrong, mitigations
10. **Open Questions** — Decisions that need user input

---

## Reference Files in ZIP

Pay special attention to these files for patterns and existing implementations:

| File | Why |
|------|-----|
| `src-tauri/src/domains.rs` | Existing domain CRUD patterns |
| `src-tauri/src/obsidian/` | Obsidian parsing (if exists) |
| `src/features/domains/` | Existing domain UI patterns |
| `src/features/settings/components/SettingsView.tsx` | UI patterns, form handling |
| `ios/CodexMonitorMobile/CodexMonitorMobile/Views/DomainDashboardView.swift` | iOS domain view pattern |
| `ios/CodexMonitorMobile/CodexMonitorMobile/CodexStore.swift` | iOS state management |

---

*This prompt was generated by Claude Code on January 28, 2026. The user (JMWillis) will review Pro's architecture design before implementation by Codex.*
