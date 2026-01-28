export type LifeDomain =
  | "delivery"
  | "nutrition"
  | "exercise"
  | "media"
  | "youtube"
  | "finance";

export type LifeTimeRange = "today" | "week" | "month" | "lifetime";

export type SortDirection = "asc" | "desc";

export interface LifeDomainConfig {
  id: LifeDomain;
  label: string;
  icon: string;
  accentColor: string;
}

export const LIFE_DOMAINS: LifeDomainConfig[] = [
  { id: "delivery", label: "Delivery", icon: "🚗", accentColor: "#3b82f6" },
  { id: "nutrition", label: "Nutrition", icon: "🍽️", accentColor: "#22c55e" },
  { id: "exercise", label: "Exercise", icon: "🏋️", accentColor: "#22c55e" },
  { id: "media", label: "Media", icon: "🎬", accentColor: "#8b5cf6" },
  { id: "youtube", label: "YouTube", icon: "🎥", accentColor: "#ef4444" },
  { id: "finance", label: "Finance", icon: "💸", accentColor: "#f59e0b" },
];

export interface LifeDomainState {
  timeRange: LifeTimeRange;
  filters: Record<string, string>;
  sortBy: string;
  sortDirection: SortDirection;
  expandedSections: Record<string, boolean>;
}

export interface DashboardMeta {
  domain: LifeDomain | string;
  range: LifeTimeRange | string;
  periodStart: string;
  periodEnd: string;
  generatedAt: string;
  sources: Array<"obsidian" | "supabase">;
  cacheHit?: boolean;
}

// -----------------------------
// Delivery
// -----------------------------

export interface DeliveryStats {
  totalEarnings: number;
  orderCount: number;
  activeHours: number;
  totalMiles?: number;
  hourlyRate: number;
  perMileRate: number;
  avgOrderValue?: number;
  startingAr?: number;
  endingAr?: number;
  whaleCatches?: number;
}

export interface DeliveryOrder {
  id: string;
  startedAt: string;
  merchantName: string;
  payout: number;
  miles?: number;
  durationMinutes?: number;
  platform?: "doordash" | "uber" | "grubhub" | "other";
  notes?: string;
  rewardTag?: string;
  warningTag?: string;
}

export interface MerchantStats {
  merchantName: string;
  orderCount: number;
  totalEarnings: number;
  avgPayout: number;
  avgMiles?: number;
  avgPerMile?: number;
  tier?: "S" | "A" | "B" | "C" | "D";
}

export interface DeliveryDashboard {
  meta: DashboardMeta;
  stats: DeliveryStats;
  orders: DeliveryOrder[];
  topMerchants: MerchantStats[];
}

// -----------------------------
// Media
// -----------------------------

export type MediaType = "Film" | "TV" | "Anime" | "Game" | "Book" | "YouTube";
export type MediaStatus = "Completed" | "Backlog";
export type MediaViewMode = "grid" | "list";
export type MediaSortOption = "rating" | "title" | "updated" | "type";

export interface MediaItem {
  id: string;
  title: string;
  type: MediaType;
  status: MediaStatus;
  rating?: number;
  coverUrl?: string;
  createdAt: string;
  updatedAt: string;
  completedAt?: string;
}

export interface MediaLibrary {
  meta: DashboardMeta;
  totalCount: number;
  completedCount: number;
  backlogCount: number;
  avgRating: number;
  items: MediaItem[];
}

export interface MediaFilterState {
  type: MediaType | "all";
  status: MediaStatus | "all";
  search: string;
  sort: MediaSortOption;
  viewMode: MediaViewMode;
}

// -----------------------------
// YouTube
// -----------------------------

export type PipelineStage =
  | "brain_dump"
  | "development"
  | "outline"
  | "evaluation"
  | "script"
  | "edit"
  | "published";

export type IdeaTier = "S" | "A" | "B" | "C";

export interface VideoIdea {
  id: string;
  title: string;
  tier: IdeaTier;
  stage: PipelineStage;
  thesis?: string;
  updatedAt: string;
  nextAction?: string;
}

export interface YouTubeDashboard {
  meta: DashboardMeta;
  pipelineStats: Record<PipelineStage, number>;
  sTier: VideoIdea[];
  inProgress: VideoIdea[];
}
