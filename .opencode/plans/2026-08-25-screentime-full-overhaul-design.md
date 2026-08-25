# ScreenTime Dashboard — Full Overhaul Design Spec

## Overview

Complete overhaul of the ScreenTime Dashboard application. The Rust backend tracks real activity data, but the frontend shows hardcoded mock data. This overhaul connects the UI to the backend, adds a dark mode toggle, decomposes monolithic pages into reusable components, integrates a real charting library, and implements a full blocked apps system with enforcement.

**Stack:** Tauri v2 + Svelte 5 + Tailwind CSS + SQLite + Rust backend

**Approach:** Bottom-up — foundation first (component decomposition, Svelte 5 consistency), then features layer by layer (dark mode → backend connection → charts → blocked apps).

---

## Phase 1: Component Decomposition + Svelte 5 Consistency

### Goal
Break monolithic pages into reusable components. Migrate all Svelte 4 syntax to Svelte 5.

### Components to Create

| Component | Props | Purpose |
|-----------|-------|---------|
| `StatCard` | `icon: string`, `value: string`, `label: string`, `trend?: number` | Single metric display with icon, value, label, optional trend arrow |
| `BarChart` | `data: ChartData`, `options?: ChartOptions` | Vertical bar chart wrapper (Chart.js) |
| `DonutChart` | `data: ChartData`, `options?: ChartOptions` | Ring/donut chart wrapper (Chart.js) |
| `AppUsageList` | `apps: AppUsage[]` | List of apps with time + proportional bar |
| `ProductivityChart` | `data: ChartData` | Full-width weekly productivity line chart |
| `DeepWorkTimeline` | `sessions: DeepWorkSession[]` | Timeline visualization for deep work blocks |
| `AppBlockCard` | `app: BlockedApp`, `onToggle: (id) => void`, `onRemove: (id) => void` | Single blocked app row with toggle + remove |
| `PageHeader` | `title: string`, `subtitle?: string` | Page title + subtitle (replaces TopBar usage in pages) |
| `TimeRangeSelector` | `value: TimeRange`, `onChange: (range) => void` | Day/Week/Month selector pills |

**Location:** All new components go in `src/lib/components/`.

### Svelte 5 Migration

- **`TopBar.svelte`** — Migrate from `export let title` / `export let subtitle` to `let { title, subtitle } = $props()`
- **All new components** — Use `$props()`, `$state()`, `$derived()` exclusively
- **Pages** — Update to use new components, maintain same routing structure

### Files Modified
- `src/lib/components/TopBar.svelte` — Svelte 5 migration
- `src/lib/components/Sidebar.svelte` — minor cleanup if needed
- `src/routes/+page.svelte` — decompose, import components
- `src/routes/productivity/+page.svelte` — decompose, import components
- `src/routes/blocked/+page.svelte` — decompose, import components

### Files Created
- `src/lib/components/StatCard.svelte`
- `src/lib/components/BarChart.svelte`
- `src/lib/components/DonutChart.svelte`
- `src/lib/components/AppUsageList.svelte`
- `src/lib/components/ProductivityChart.svelte`
- `src/lib/components/DeepWorkTimeline.svelte`
- `src/lib/components/AppBlockCard.svelte`
- `src/lib/components/PageHeader.svelte`
- `src/lib/components/TimeRangeSelector.svelte`

---

## Phase 2: Dark Mode Toggle

### Goal
Add a working dark mode toggle. Default: light. Persisted in localStorage.

### Architecture

**New file: `src/lib/stores/theme.ts`**
```
- State: current theme ('light' | 'dark')
- Initialize from localStorage, default 'light'
- toggle() — switches theme, updates localStorage, syncs DOM class
- On init: apply saved theme to document.documentElement
```

**New file: `src/lib/components/ThemeToggle.svelte`**
- Sun/moon icon button
- Calls theme store's toggle()
- Placed in TopBar, top-right area

### CSS Updates

**`src/app.css`:**
- Add `.dark` class overrides for glassmorphic tokens
- Ensure all `dark:` Tailwind variants are properly wired

**All pages + components:**
- Add `dark:bg-*`, `dark:text-*`, `dark:border-*`, `dark:backdrop-blur-*` variants
- Use the M3 color tokens already defined in `tailwind.config.js`

### Files Modified
- `src/app.css` — dark mode overrides
- `src/routes/+page.svelte` — dark variants
- `src/routes/productivity/+page.svelte` — dark variants
- `src/routes/blocked/+page.svelte` — dark variants
- `src/lib/components/Sidebar.svelte` — dark variants
- `src/lib/components/TopBar.svelte` — dark variants + ThemeToggle placement
- All new components from Phase 1 — dark variants

### Files Created
- `src/lib/stores/theme.ts`
- `src/lib/components/ThemeToggle.svelte`

---

## Phase 3: Backend Connection

### Goal
Replace all hardcoded mock data with real data from the existing backend.

### Current Store (already built, unused)

`src/lib/stores/activities.ts` provides:
- `fetchActivities(start, end)` → calls `invoke('get_activities')`
- `totalDuration` (derived) — sum of all activity durations
- `productivityScore` (derived) — percentage of productive time
- `deepWorkSessions` (derived) — count of sessions > 25 minutes
- `formatDuration(minutes)` — human-readable duration string

### New Backend Commands (Rust)

**`get_daily_summary(start_date, end_date)`**
Returns per-day aggregates: total screen time, productive percentage, top 5 apps.

New Rust struct:
```rust
struct DailySummary {
    date: String,
    total_minutes: i64,
    productive_minutes: i64,
    productive_percentage: f64,
    top_apps: Vec<AppUsage>,
}

struct AppUsage {
    app_name: String,
    duration_minutes: i64,
    category: String,
}
```

**`get_productivity_by_day(start_date, end_date)`**
Returns daily productivity percentage for chart rendering.

**`get_deep_work_sessions(start_date, end_date)`**
Returns sessions where the user focused on a single app for > 25 minutes continuously.

New Rust struct:
```rust
struct DeepWorkSession {
    start_time: String,
    end_time: String,
    duration_minutes: i64,
    app_name: String,
}
```

### Frontend Wiring

**Overview page (`+page.svelte`):**
- `StatCard` for "Total Screen Time" ← `totalDuration` store
- `StatCard` for "Productivity Score" ← `productivityScore` store
- `BarChart` for daily breakdown ← `get_daily_summary`
- `AppUsageList` for top apps ← `activities` store

**Productivity page (`productivity/+page.svelte`):**
- `StatCard` for "Deep Work Sessions" ← `deepWorkSessions` store
- `ProductivityChart` for weekly trend ← `get_productivity_by_day`
- `DeepWorkTimeline` for session blocks ← `get_deep_work_sessions`

**Time range reactivity:**
- `TimeRangeSelector` updates a shared `timeRange` state
- All data fetches react to range changes
- Default: last 7 days

### Files Modified
- `src-tauri/src/lib.rs` — new commands, new structs
- `src/lib/stores/activities.ts` — add new derived stores
- `src/routes/+page.svelte` — wire to real data
- `src/routes/productivity/+page.svelte` — wire to real data
- `src/routes/+layout.svelte` — pass time range to pages

### Files Created
- `src/lib/stores/timeRange.ts` — shared time range state

---

## Phase 4: Real Charting Library

### Goal
Replace decorative HTML charts with interactive Chart.js visualizations.

### Dependencies
- `chart.js` — core library
- `svelte-chartjs` — Svelte wrapper

Install: `npm install chart.js svelte-chartjs`

### Charts

**Daily Screen Time Bar Chart (Overview):**
- Type: `bar`
- X-axis: days (Mon–Sun)
- Y-axis: hours
- Color: primary M3 token
- Responsive, no grid lines, minimal styling

**App Usage Donut Chart (Overview):**
- Type: `doughnut`
- Segments: top 5 apps by usage + "Other"
- Colors: M3 secondary/tertiary tokens
- Center text: total screen time
- Legend: app names with time

**Weekly Productivity Line Chart (Productivity):**
- Type: `line`
- X-axis: days
- Y-axis: percentage (0–100%)
- Fill: gradient below line
- Color: green M3 token
- Smooth curve

**Deep Work Blocks (Productivity):**
- Type: `bar` (horizontal)
- X-axis: hours
- Y-axis: sessions
- Color: tertiary M3 token
- Each block shows app name

### Styling
- Charts wrapped in glassmorphic containers (`backdrop-blur-sm rounded-2xl`)
- Use Tailwind M3 tokens for all colors
- Responsive — full width on desktop, stacked on mobile
- No chartjs-plugin-gradient — keep dependencies minimal

### Files Modified
- `src/lib/components/BarChart.svelte` — Chart.js integration
- `src/lib/components/DonutChart.svelte` — Chart.js integration
- `src/lib/components/ProductivityChart.svelte` — Chart.js integration
- `src/routes/+page.svelte` — chart data wiring
- `src/routes/productivity/+page.svelte` — chart data wiring

### Files Created
- None (components already exist from Phase 1, just need Chart.js implementation)

---

## Phase 5: Blocked Apps System

### Goal
Full blocked apps management: database storage, backend commands, frontend UI, and actual blocking enforcement via window manager rules.

### Database

New `blocked_apps` table in SQLite:
```sql
CREATE TABLE blocked_apps (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    app_name TEXT NOT NULL UNIQUE,
    bundle_id TEXT,
    is_active INTEGER DEFAULT 1,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

### Backend Commands (Rust)

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `get_blocked_apps` | none | `Vec<BlockedApp>` | List all blocked apps |
| `add_blocked_app` | `app_name: String` | `Vec<BlockedApp>` | Add app to block list |
| `remove_blocked_app` | `app_id: i64` | `Vec<BlockedApp>` | Remove from block list |
| `toggle_blocked_app` | `(app_id: i64, is_active: bool)` | `Vec<BlockedApp>` | Toggle active state |

New Rust struct:
```rust
struct BlockedApp {
    id: i64,
    app_name: String,
    is_active: bool,
}
```

### Blocking Enforcement

**New module: `src-tauri/src/blocker.rs`**

Called from `tracker.rs` polling loop after detecting foreground window:

1. Check if detected app matches any active blocked app
2. If match found:
   - **Hyprland (Wayland):** `hyprctl dispatch closewindow` or `hyprctl dispatch focuswindow` away
   - **Sway (Wayland):** `[app_id="..."] kill`
   - **X11:** `xdotool windowclose` or `wmctrl -c`
3. If no WM detected: log warning, operate in "tracking only" mode

**Detection method:** Compare foreground window class/name against blocked app names (case-insensitive substring match).

### Frontend

**Rewritten: `src/routes/blocked/+page.svelte`**
- `PageHeader` with title "Blocked Apps"
- `AppBlockCard` list — shows all blocked apps with toggle + remove
- "Add App" button → opens `AddAppModal`
- Empty state when no apps blocked

**New: `src/lib/components/AddAppModal.svelte`**
- Search input to find apps
- Shows recently used apps from activity data
- Add button to block selected app
- Modal overlay with glassmorphic styling

**New store: `src/lib/stores/blockedApps.ts`**
- `fetchBlockedApps()` → invoke `get_blocked_apps`
- `addBlockedApp(name)` → invoke `add_blocked_app`
- `removeBlockedApp(id)` → invoke `remove_blocked_app`
- `toggleBlockedApp(id, active)` → invoke `toggle_blocked_app`

### Files Modified
- `src-tauri/src/lib.rs` — new commands, new struct, add `blocked_apps` table to `init_db()`
- `src-tauri/src/tracker.rs` — integrate blocker check
- `src/routes/blocked/+page.svelte` — full rewrite

### Files Created
- `src-tauri/src/blocker.rs`
- `src/lib/stores/blockedApps.ts`
- `src/lib/components/AddAppModal.svelte`

---

## Testing Strategy

### Backend
- Unit tests for new Rust commands (mock SQLite)
- Integration tests for blocker module (mock window manager calls)
- Test DB migration (blocked_apps table creation)

### Frontend
- Visual testing: verify all pages render with real data
- Dark mode: verify all components respect theme
- Responsive: verify layout at different viewport sizes
- Accessibility: keyboard navigation, screen reader support

### Manual Verification
- Run `cargo test` for Rust tests
- Run `npm run check` for Svelte type checking
- Run `npm run build` for production build verification
- Manual testing: start app, verify tracking, verify blocking

---

## Execution Order

1. **Phase 1** — Component decomposition + Svelte 5 (foundation)
2. **Phase 2** — Dark mode toggle (independent, quick)
3. **Phase 3** — Backend connection (core functionality)
4. **Phase 4** — Charting library (needs real data from Phase 3)
5. **Phase 5** — Blocked apps system (most complex, built on working foundation)

Each phase is independently testable and deployable. The app remains functional between phases.
