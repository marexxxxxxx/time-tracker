# Dark Mode & Settings Overhaul — Design Spec

## Overview

Complete overhaul of the Screen Time Tracker's dark mode system and activation of all placeholder UI elements. The goal is a polished, production-ready dark mode with a centralized color system, and every button in the UI performing a meaningful function.

**Current State:**
- Dark mode uses ad-hoc `dark:` Tailwind classes scattered across 14 files
- No centralized dark color palette in Tailwind config
- Charts always use hardcoded light-mode colors
- 4 TopBar buttons are placeholders (Calendar, Tune, Share, Add Limit)
- Settings and Help sidebar links are dead (`href="#"`)
- `settings` DB table exists but is never used

**Stack:** Tauri v2 + Svelte 5 + Tailwind CSS + SQLite + Rust backend

---

## 1. Dark Mode Overhaul

### 1.1 Centralized Dark Palette

Add M3 dark theme tokens to `tailwind.config.js` using Tailwind's `darkMode: "class"` strategy. The dark palette uses the inverse surface approach from M3:

```js
// Key dark tokens (M3 Dark Theme)
surface: '#1a1b1f'           // was #faf9fe in light
on-surface: '#e3e2e7'        // was #1a1b1f in light
surface-dim: '#131316'       // was #dad9df
surface-container-lowest: '#0f0f12' // was #ffffff
surface-container-low: '#1a1b1f'    // was #f4f3f8
surface-container: '#202124'        // was #eeedf3
surface-container-high: '#2b2c2f'   // was #e9e7ed
surface-container-highest: '#363739' // was #e3e2e7
primary: '#adc6ff'           // was #0058bc (lighter for dark bg)
on-primary: '#002f65'        // was #ffffff
secondary: '#88d86a'         // was #006e28
tertiary: '#c2c1ff'          // was #4c4aca
error: '#ffb4ab'             // was #ba1a1a
outline: '#8e9099'           // was #717786
outline-variant: '#44474f'   // was #c1c6d7
inverse-surface: '#e3e2e7'   // was #2f3034
inverse-on-surface: '#1a1b1f' // was #f1f0f5
```

### 1.2 Component Cleanup

After centralized tokens are in place, remove ad-hoc `dark:` prefixed classes from all 14 component files. The components already use token names like `bg-surface`, `text-on-surface` — the dark mode will "just work" via the Tailwind dark palette.

**Files to clean:**
- `src/lib/components/Sidebar.svelte`
- `src/lib/components/TopBar.svelte`
- `src/lib/components/StatCard.svelte`
- `src/lib/components/TimeRangeSelector.svelte`
- `src/lib/components/AppUsageList.svelte`
- `src/lib/components/AppBlockCard.svelte`
- `src/lib/components/AddAppModal.svelte`
- `src/lib/components/LimitEditor.svelte`
- `src/lib/components/DeepWorkTimeline.svelte`
- `src/routes/+page.svelte`
- `src/routes/blocked/+page.svelte`
- `src/routes/productivity/+page.svelte`

### 1.3 Chart Dark Mode

Charts (BarChart, DonutChart, CategoryDonut, ProductivityChart) use hardcoded light colors. Add CSS custom properties for chart colors that switch in dark mode:

```css
:root {
  --chart-primary: #0058bc;
  --chart-secondary: #006e28;
  --chart-tertiary: #4c4aca;
  --chart-error: #E50914;
  --chart-neutral: #e3e2e7;
  --chart-grid: #c1c6d7;
  --chart-text: #1a1b1f;
}
.dark {
  --chart-primary: #adc6ff;
  --chart-secondary: #88d86a;
  --chart-tertiary: #c2c1ff;
  --chart-error: #ffb4ab;
  --chart-neutral: #363739;
  --chart-grid: #44474f;
  --chart-text: #e3e2e7;
}
```

Chart components read from these CSS variables instead of hardcoded values.

### 1.4 ThemeToggle Enhancement

Currently toggles light/dark. Extend to support 3 modes:
- **System** (follows OS preference via `prefers-color-scheme`)
- **Light**
- **Dark**

Persisted in localStorage. The toggle cycles: System → Light → Dark → System.

### 1.5 Theme Store Update

Extend `src/lib/stores/theme.ts`:
- Type changes from `'light' | 'dark'` to `'system' | 'light' | 'dark'`
- On `system`, listen to `matchMedia('(prefers-color-scheme: dark)')` for live updates
- On `light`/`dark`, force that mode

---

## 2. TopBar Buttons

### 2.1 Calendar Button (`calendar_today`)

**Function:** Opens a date picker popover to navigate to a specific date.

**Behavior:**
- Click opens a minimal calendar popover (built with HTML date input or simple grid)
- Selecting a date updates the `timeRange` store and fetches data for that date
- Current date shown as subtitle badge in TopBar

**Files:** New `DatePickerPopover.svelte` component, update `TopBar.svelte`

### 2.2 Tune/Filter Button (`tune`)

**Function:** Opens a filter popover for the current view.

**Behavior:**
- Click opens a dropdown with category checkboxes (Coding, Design, Communication, Entertainment, Neutral)
- Filtering applies to the current page's data display
- Active filter count shown as badge on the icon

**Files:** New `FilterPopover.svelte` component, update `TopBar.svelte`, update stores to support filtering

### 2.3 Share Button

**Function:** Exports the current view's data.

**Behavior:**
- Click opens a small dropdown: "Export CSV" / "Export JSON"
- Generates file from current activities data
- Uses Tauri's save dialog or browser download

**Files:** New `ExportDropdown.svelte` component, update `TopBar.svelte`, new Rust command for data export

### 2.4 "Add Limit" Button

**Function:** Quick-add a time limit for any app.

**Behavior:**
- Click opens a modal with: App selector (dropdown/search), Daily limit input, Weekly limit input
- Saves via existing `update_app_limits` Tauri command
- Shows success toast

**Files:** New `QuickAddLimitModal.svelte` component, update `TopBar.svelte`

---

## 3. Settings Page

### 3.1 Route & Layout

New route: `/settings` → `src/routes/settings/+page.svelte`

Sidebar link changes from `href="#"` to `href="/settings"` with active state support.

### 3.2 Settings Sections

#### Appearance
- **Theme:** System / Light / Dark (radio buttons, synced with theme store)
- **Accent Color:** Primary blue (could extend to multiple accent colors in future)

#### Tracker
- **Idle Detection Timeout:** 5 / 10 / 15 / 30 minutes (dropdown) — default 5min
- **Tracking Paused:** Toggle to pause all tracking temporarily

#### Notifications
- **Limit Warnings:** Toggle — show warning when approaching app limit
- **Daily Summary:** Toggle — show end-of-day summary notification

#### Data Management
- **Export All Data:** Button → downloads CSV/JSON of all activities
- **Clear All Data:** Button → confirmation dialog → deletes all activities
- **Reset Demo Data:** Button → re-seeds demo data

#### About
- Version number
- "Screen Time Tracker" branding
- Links to GitHub repo

### 3.3 Backend (Rust)

Extend `settings` table usage with new Tauri commands:
- `get_settings` — returns all settings as key-value map
- `update_setting` — updates a single setting
- `export_data` — returns all activities as JSON
- `clear_all_data` — deletes all activities
- `reset_demo_data` — re-seeds the database

### 3.4 Settings Store

New `src/lib/stores/settings.ts`:
- Svelte store for all settings
- Loads from backend on mount
- Auto-saves changes to backend
- Individual derived stores for specific settings (e.g., `idleTimeout`, `themePreference`)

---

## 4. Help Page

### 4.1 Route

New route: `/help` → `src/routes/help/+page.svelte`

Sidebar link changes from `href="#"` to `href="/help"` with active state support.

### 4.2 Content

Simple FAQ-style page:
- **Keyboard Shortcuts:** (none yet, but placeholder for future)
- **How Tracking Works:** Brief explanation of window polling
- **Blocked Apps:** How blocking and limits work
- **Data Privacy:** All data stays local, no cloud sync
- **Support:** GitHub issues link

---

## 5. Files Modified/Created

### Modified
| File | Changes |
|------|---------|
| `tailwind.config.js` | Add dark color palette |
| `src/app.css` | Add CSS variables for charts (light + dark) |
| `src/lib/stores/theme.ts` | Support system/light/dark, media query listener |
| `src/lib/components/ThemeToggle.svelte` | 3-mode cycling toggle |
| `src/lib/components/TopBar.svelte` | Wire up all 4 buttons |
| `src/lib/components/Sidebar.svelte` | Active states for Settings/Help, remove dark: classes |
| `src/lib/components/StatCard.svelte` | Remove dark: classes |
| `src/lib/components/TimeRangeSelector.svelte` | Remove dark: classes |
| `src/lib/components/AppUsageList.svelte` | Remove dark: classes |
| `src/lib/components/AppBlockCard.svelte` | Remove dark: classes |
| `src/lib/components/AddAppModal.svelte` | Remove dark: classes |
| `src/lib/components/LimitEditor.svelte` | Remove dark: classes |
| `src/lib/components/DeepWorkTimeline.svelte` | Remove dark: classes |
| `src/components/BarChart.svelte` | Use CSS variables for colors |
| `src/components/DonutChart.svelte` | Use CSS variables for colors |
| `src/components/CategoryDonut.svelte` | Use CSS variables for colors |
| `src/components/ProductivityChart.svelte` | Use CSS variables for colors |
| `src/routes/+page.svelte` | Remove dark: classes |
| `src/routes/blocked/+page.svelte` | Remove dark: classes |
| `src/routes/productivity/+page.svelte` | Remove dark: classes |
| `src-tauri/src/lib.rs` | Add settings + export Tauri commands |

### Created
| File | Purpose |
|------|---------|
| `src/routes/settings/+page.svelte` | Settings page |
| `src/routes/help/+page.svelte` | Help page |
| `src/lib/stores/settings.ts` | Settings store |
| `src/lib/components/DatePickerPopover.svelte` | Calendar date picker |
| `src/lib/components/FilterPopover.svelte` | Category filter dropdown |
| `src/lib/components/ExportDropdown.svelte` | Data export dropdown |
| `src/lib/components/QuickAddLimitModal.svelte` | Quick time limit modal |

---

## 6. Testing

- Visual verification: toggle dark mode on every page
- Chart colors adapt to theme
- All TopBar buttons open correct UI and perform actions
- Settings persist across app restart (via DB)
- Settings page reads/writes correctly
- Help page renders

---

## 7. Out of Scope

- Multiple accent colors (future enhancement)
- System tray integration
- Notifications via OS (requires additional Tauri plugins)
- Keyboard shortcuts system
