# Dark Mode & Settings Overhaul — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Centralize dark mode via M3 Tailwind palette, activate all placeholder buttons, add Settings and Help pages.

**Architecture:** Bottom-up — foundation first (Tailwind dark palette, CSS variables, theme store), then component cleanup, then new features (TopBar buttons, Settings page, Help page). Each task is independently testable.

**Tech Stack:** Tauri v2, Svelte 5 ($state/$derived/$props), Tailwind CSS (class-based dark mode), Chart.js, SQLite (rusqlite), Rust

**Spec:** `.opencode/plans/2026-08-26-dark-mode-and-settings-overhaul.md`

## Global Constraints

- Dark mode: `darkMode: "class"` in Tailwind — activated by `.dark` class on `<html>`
- Svelte 5 only: use `$props()`, `$state()`, `$derived()` — no `export let`
- M3 color tokens: use existing token names (`surface`, `on-surface`, `primary`, etc.)
- Tauri commands: registered in `src-tauri/src/lib.rs` via `tauri::generate_handler![]`
- Database: SQLite at `$APP_DATA_DIR/screentime.db`, existing `settings` table (key TEXT, value TEXT)

---

## File Structure

### Modified
| File | Responsibility |
|------|---------------|
| `tailwind.config.js` | Dark color palette (M3 dark tokens) |
| `src/app.css` | CSS variables for chart colors (light + dark) |
| `src/lib/stores/theme.ts` | system/light/dark mode with media query listener |
| `src/lib/components/ThemeToggle.svelte` | 3-mode cycling toggle (System→Light→Dark) |
| `src/lib/components/TopBar.svelte` | Wire Calendar, Tune, Share, Add Limit buttons |
| `src/lib/components/Sidebar.svelte` | Active route for Settings/Help, remove dark: classes |
| `src/lib/components/StatCard.svelte` | Remove ad-hoc dark: classes |
| `src/lib/components/TimeRangeSelector.svelte` | Remove ad-hoc dark: classes |
| `src/lib/components/AppUsageList.svelte` | Remove ad-hoc dark: classes |
| `src/lib/components/AppBlockCard.svelte` | Remove ad-hoc dark: classes |
| `src/lib/components/AddAppModal.svelte` | Remove ad-hoc dark: classes |
| `src/lib/components/LimitEditor.svelte` | Remove ad-hoc dark: classes |
| `src/lib/components/DeepWorkTimeline.svelte` | Remove ad-hoc dark: classes |
| `src/lib/components/BarChart.svelte` | Use CSS variables for colors |
| `src/lib/components/DonutChart.svelte` | Use CSS variables for colors |
| `src/lib/components/CategoryDonut.svelte` | Use CSS variables for colors |
| `src/lib/components/ProductivityChart.svelte` | Use CSS variables for colors |
| `src/routes/+page.svelte` | Remove dark: classes |
| `src/routes/blocked/+page.svelte` | Remove dark: classes |
| `src/routes/productivity/+page.svelte` | Remove dark: classes |
| `src-tauri/src/lib.rs` | Add settings + export Tauri commands |

### Created
| File | Responsibility |
|------|---------------|
| `src/routes/settings/+page.svelte` | Settings page UI |
| `src/routes/help/+page.svelte` | Help/FAQ page UI |
| `src/lib/stores/settings.ts` | Settings CRUD store |
| `src/lib/components/DatePickerPopover.svelte` | Calendar date picker popover |
| `src/lib/components/FilterPopover.svelte` | Category filter dropdown |
| `src/lib/components/ExportDropdown.svelte` | Data export dropdown |
| `src/lib/components/QuickAddLimitModal.svelte` | Quick time limit modal |

---

## Tasks

### Task 1: Dark Mode Foundation — Tailwind Config + CSS Variables

**Files:**
- Modify: `tailwind.config.js` (add dark palette under `darkMode` key or as `.dark` overrides)
- Modify: `src/app.css` (add CSS variables for chart colors)

**Interfaces:**
- Consumes: existing M3 token names in `tailwind.config.js`
- Produces: dark color tokens accessible via `dark:bg-surface`, `dark:text-on-surface` etc.; CSS variables `--chart-primary` etc.

- [ ] **Step 1: Add dark color palette to tailwind.config.js**

Replace the `colors` object in `tailwind.config.js`. The approach: define a `dark` key inside `extend.colors` that maps to M3 dark tokens, OR define a second set of tokens that activate when `.dark` is on `<html>`.

The cleanest approach for Tailwind `darkMode: "class"`: keep the existing light tokens, and add a `dark` namespace OR override individual tokens. Since Tailwind doesn't natively switch token values based on dark mode, we use CSS custom properties:

In `tailwind.config.js`, replace the hardcoded color values with CSS variable references:

```js
"colors": {
    "primary": "var(--color-primary)",
    "on-primary": "var(--color-on-primary)",
    "primary-container": "var(--color-primary-container)",
    "on-primary-container": "var(--color-on-primary-container)",
    "secondary": "var(--color-secondary)",
    "on-secondary": "var(--color-on-secondary)",
    "secondary-container": "var(--color-secondary-container)",
    "on-secondary-container": "var(--color-on-secondary-container)",
    "tertiary": "var(--color-tertiary)",
    "on-tertiary": "var(--color-on-tertiary)",
    "tertiary-container": "var(--color-tertiary-container)",
    "on-tertiary-container": "var(--color-on-tertiary-container)",
    "error": "var(--color-error)",
    "on-error": "var(--color-on-error)",
    "error-container": "var(--color-error-container)",
    "on-error-container": "var(--color-on-error-container)",
    "surface": "var(--color-surface)",
    "surface-dim": "var(--color-surface-dim)",
    "surface-bright": "var(--color-surface-bright)",
    "surface-container-lowest": "var(--color-surface-container-lowest)",
    "surface-container-low": "var(--color-surface-container-low)",
    "surface-container": "var(--color-surface-container)",
    "surface-container-high": "var(--color-surface-container-high)",
    "surface-container-highest": "var(--color-surface-container-highest)",
    "on-surface": "var(--color-on-surface)",
    "on-surface-variant": "var(--color-on-surface-variant)",
    "surface-variant": "var(--color-surface-variant)",
    "surface-tint": "var(--color-surface-tint)",
    "inverse-surface": "var(--color-inverse-surface)",
    "inverse-on-surface": "var(--color-inverse-on-surface)",
    "inverse-primary": "var(--color-inverse-primary)",
    "outline": "var(--color-outline)",
    "outline-variant": "var(--color-outline-variant)",
    "background": "var(--color-background)",
    "on-background": "var(--color-on-background)",
    "primary-fixed": "var(--color-primary-fixed)",
    "primary-fixed-dim": "var(--color-primary-fixed-dim)",
    "on-primary-fixed": "var(--color-on-primary-fixed)",
    "on-primary-fixed-variant": "var(--color-on-primary-fixed-variant)",
    "secondary-fixed": "var(--color-secondary-fixed)",
    "secondary-fixed-dim": "var(--color-secondary-fixed-dim)",
    "on-secondary-fixed": "var(--color-on-secondary-fixed)",
    "on-secondary-fixed-variant": "var(--color-on-secondary-fixed-variant)",
    "tertiary-fixed": "var(--color-tertiary-fixed)",
    "tertiary-fixed-dim": "var(--color-tertiary-fixed-dim)",
    "on-tertiary-fixed": "var(--color-on-tertiary-fixed)",
    "on-tertiary-fixed-variant": "var(--color-on-tertiary-fixed-variant)",
}
```

- [ ] **Step 2: Add CSS custom properties to src/app.css**

Add light and dark theme variables at the top of `src/app.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
    /* M3 Light Theme */
    --color-primary: #0058bc;
    --color-on-primary: #ffffff;
    --color-primary-container: #0070eb;
    --color-on-primary-container: #fefcff;
    --color-secondary: #006e28;
    --color-on-secondary: #ffffff;
    --color-secondary-container: #6ffb85;
    --color-on-secondary-container: #00732a;
    --color-tertiary: #4c4aca;
    --color-on-tertiary: #ffffff;
    --color-tertiary-container: #6664e4;
    --color-on-tertiary-container: #fffbff;
    --color-error: #ba1a1a;
    --color-on-error: #ffffff;
    --color-error-container: #ffdad6;
    --color-on-error-container: #93000a;
    --color-surface: #faf9fe;
    --color-surface-dim: #dad9df;
    --color-surface-bright: #faf9fe;
    --color-surface-container-lowest: #ffffff;
    --color-surface-container-low: #f4f3f8;
    --color-surface-container: #eeedf3;
    --color-surface-container-high: #e9e7ed;
    --color-surface-container-highest: #e3e2e7;
    --color-on-surface: #1a1b1f;
    --color-on-surface-variant: #414755;
    --color-surface-variant: #e3e2e7;
    --color-surface-tint: #005bc1;
    --color-inverse-surface: #2f3034;
    --color-inverse-on-surface: #f1f0f5;
    --color-inverse-primary: #adc6ff;
    --color-outline: #717786;
    --color-outline-variant: #c1c6d7;
    --color-background: #faf9fe;
    --color-on-background: #1a1b1f;
    --color-primary-fixed: #d8e2ff;
    --color-primary-fixed-dim: #adc6ff;
    --color-on-primary-fixed: #001a41;
    --color-on-primary-fixed-variant: #004493;
    --color-secondary-fixed: #72fe88;
    --color-secondary-fixed-dim: #53e16f;
    --color-on-secondary-fixed: #002107;
    --color-on-secondary-fixed-variant: #00531c;
    --color-tertiary-fixed: #e2dfff;
    --color-tertiary-fixed-dim: #c2c1ff;
    --color-on-tertiary-fixed: #0c006a;
    --color-on-tertiary-fixed-variant: #3631b4;

    /* Chart colors (light) */
    --chart-primary: #0058bc;
    --chart-secondary: #006e28;
    --chart-tertiary: #4c4aca;
    --chart-error: #E50914;
    --chart-neutral: #e3e2e7;
    --chart-grid: #c1c6d7;
    --chart-text: #1a1b1f;
}

.dark {
    /* M3 Dark Theme */
    --color-primary: #adc6ff;
    --color-on-primary: #002f65;
    --color-primary-container: #00459a;
    --color-on-primary-container: #d8e2ff;
    --color-secondary: #88d86a;
    --color-on-secondary: #003910;
    --color-secondary-container: #00531c;
    --color-on-secondary-container: #72fe88;
    --color-tertiary: #c2c1ff;
    --color-on-tertiary: #1c0090;
    --color-tertiary-container: #3631b4;
    --color-on-tertiary-container: #e2dfff;
    --color-error: #ffb4ab;
    --color-on-error: #690005;
    --color-error-container: #93000a;
    --color-on-error-container: #ffdad6;
    --color-surface: #1a1b1f;
    --color-surface-dim: #131316;
    --color-surface-bright: #3a3a3d;
    --color-surface-container-lowest: #0f0f12;
    --color-surface-container-low: #1a1b1f;
    --color-surface-container: #202124;
    --color-surface-container-high: #2b2c2f;
    --color-surface-container-highest: #363739;
    --color-on-surface: #e3e2e7;
    --color-on-surface-variant: #c4c6d0;
    --color-surface-variant: #44474f;
    --color-surface-tint: #adc6ff;
    --color-inverse-surface: #e3e2e7;
    --color-inverse-on-surface: #1a1b1f;
    --color-inverse-primary: #0058bc;
    --color-outline: #8e9099;
    --color-outline-variant: #44474f;
    --color-background: #1a1b1f;
    --color-on-background: #e3e2e7;
    --color-primary-fixed: #d8e2ff;
    --color-primary-fixed-dim: #adc6ff;
    --color-on-primary-fixed: #001a41;
    --color-on-primary-fixed-variant: #004493;
    --color-secondary-fixed: #72fe88;
    --color-secondary-fixed-dim: #53e16f;
    --color-on-secondary-fixed: #002107;
    --color-on-secondary-fixed-variant: #00531c;
    --color-tertiary-fixed: #e2dfff;
    --color-tertiary-fixed-dim: #c2c1ff;
    --color-on-tertiary-fixed: #0c006a;
    --color-on-tertiary-fixed-variant: #3631b4;

    /* Chart colors (dark) */
    --chart-primary: #adc6ff;
    --chart-secondary: #88d86a;
    --chart-tertiary: #c2c1ff;
    --chart-error: #ffb4ab;
    --chart-neutral: #363739;
    --chart-grid: #44474f;
    --chart-text: #e3e2e7;
}

@layer base {
    body {
        @apply bg-background text-on-surface antialiased min-h-screen;
    }
}
```

- [ ] **Step 3: Verify dark mode works**

Run: `cd screen-time-app && npm run dev`

In browser, open dev tools, manually add `class="dark"` to `<html>` — verify colors switch.

- [ ] **Step 4: Commit**

```bash
git add tailwind.config.js src/app.css
git commit -m "feat: centralize dark mode via CSS custom properties + M3 palette"
```

---

### Task 2: Theme Store — System/Light/Dark Support

**Files:**
- Modify: `src/lib/stores/theme.ts`

**Interfaces:**
- Consumes: none
- Produces: `theme` store with `.subscribe`, `.toggle()`, `.set(value)` where value is `'system' | 'light' | 'dark'`

- [ ] **Step 1: Rewrite theme.ts**

```typescript
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

type Theme = 'system' | 'light' | 'dark';

function getSystemTheme(): 'light' | 'dark' {
    if (!browser) return 'light';
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function applyTheme(theme: Theme) {
    if (!browser) return;
    const effective = theme === 'system' ? getSystemTheme() : theme;
    document.documentElement.classList.toggle('dark', effective === 'dark');
}

function createThemeStore() {
    const stored = browser ? localStorage.getItem('theme') as Theme | null : null;
    const initial = stored ?? 'system';
    const { subscribe, set, update } = writable<Theme>(initial);

    if (browser) {
        applyTheme(initial);

        // Listen for system theme changes when in 'system' mode
        const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
        mediaQuery.addEventListener('change', () => {
            const current = localStorage.getItem('theme') as Theme;
            if (current === 'system' || !current) {
                applyTheme('system');
            }
        });
    }

    return {
        subscribe,
        toggle: () => {
            update(current => {
                const cycle: Theme[] = ['system', 'light', 'dark'];
                const nextIndex = (cycle.indexOf(current) + 1) % cycle.length;
                const next = cycle[nextIndex];
                if (browser) {
                    localStorage.setItem('theme', next);
                    applyTheme(next);
                }
                return next;
            });
        },
        set: (value: Theme) => {
            set(value);
            if (browser) {
                localStorage.setItem('theme', value);
                applyTheme(value);
            }
        }
    };
}

export const theme = createThemeStore();
```

- [ ] **Step 2: Verify toggle works**

Run: `cd screen-time-app && npm run dev`

Click ThemeToggle — should cycle System → Light → Dark → System. Verify:
- System mode follows OS preference
- Light mode forces light
- Dark mode forces dark

- [ ] **Step 3: Commit**

```bash
git add src/lib/stores/theme.ts
git commit -m "feat: theme store supports system/light/dark with OS media query"
```

---

### Task 3: ThemeToggle — 3-Mode Cycling

**Files:**
- Modify: `src/lib/components/ThemeToggle.svelte`

**Interfaces:**
- Consumes: `theme` store from Task 2
- Produces: renders icon based on current mode (system_stats, light_mode, dark_mode)

- [ ] **Step 1: Rewrite ThemeToggle.svelte**

```svelte
<script lang="ts">
    import { theme } from '$lib/stores/theme';

    let currentTheme = $derived($theme);

    function cycleTheme() {
        theme.toggle();
    }

    let icon = $derived(currentTheme === 'system' ? 'system_stats' : currentTheme === 'light' ? 'light_mode' : 'dark_mode');
    let label = $derived(currentTheme === 'system' ? 'System' : currentTheme === 'light' ? 'Light' : 'Dark');
</script>

<button
    class="text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low p-sm rounded-full transition-colors flex items-center justify-center"
    onclick={cycleTheme}
    title="Theme: {label}"
>
    <span class="material-symbols-outlined">{icon}</span>
</button>
```

- [ ] **Step 2: Verify**

Run dev server, click toggle — icon should cycle: system_stats → light_mode → dark_mode → system_stats.

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/ThemeToggle.svelte
git commit -m "feat: ThemeToggle cycles system/light/dark with visual feedback"
```

---

### Task 4: Component Cleanup — Remove Ad-hoc Dark Classes

**Files:**
- Modify: all 14 files listed in spec section 1.2

**Interfaces:**
- Consumes: CSS variables from Task 1 (dark palette auto-applies via `.dark` class)
- Produces: components using only M3 token names, no `dark:` prefixes

- [ ] **Step 1: Clean Sidebar.svelte**

Remove all `dark:` prefixed classes. The component uses `bg-surface/80 dark:bg-inverse-surface/80` — change to `bg-surface/80` since the CSS variable will handle dark automatically. Similar pattern for all other `dark:` variants.

Key replacements in Sidebar.svelte:
- `dark:bg-inverse-surface/80` → remove (surface already handles dark)
- `dark:text-primary-fixed-dim` → remove (text-primary already handles dark)
- `dark:text-surface-variant` → remove (text-on-surface-variant already handles dark)
- `dark:hover:bg-surface-container-highest/20` → remove (hover already uses token)
- `dark:border-outline/20` → remove (border-outline-variant/20 already handles dark)

- [ ] **Step 2: Clean TopBar.svelte**

Remove `dark:bg-inverse-surface/60`, `dark:text-on-surface`, `dark:border-outline/20` — tokens handle dark automatically.

- [ ] **Step 3: Clean StatCard.svelte**

Remove `dark:bg-surface-container` — bg-surface-container-lowest with dark token handles it.

- [ ] **Step 4: Clean remaining components**

For each file in the list (TimeRangeSelector, AppUsageList, AppBlockCard, AddAppModal, LimitEditor, DeepWorkTimeline, and the 3 route pages):
- Remove every `dark:` prefixed class
- Verify the base token still applies correctly

- [ ] **Step 5: Verify dark mode on every page**

Run dev server, toggle dark mode, visit every page:
- Overview: cards, charts, progress bars
- Blocked Apps: sections, cards, modals
- Productivity: cards, chart, timeline

- [ ] **Step 6: Commit**

```bash
git add src/lib/components/*.svelte src/routes/**/*.svelte
git commit -m "feat: remove ad-hoc dark: classes — centralized palette handles all themes"
```

---

### Task 5: Chart Dark Mode — CSS Variables

**Files:**
- Modify: `src/lib/components/BarChart.svelte`
- Modify: `src/lib/components/DonutChart.svelte`
- Modify: `src/lib/components/CategoryDonut.svelte`
- Modify: `src/lib/components/ProductivityChart.svelte`

**Interfaces:**
- Consumes: CSS variables `--chart-primary`, `--chart-secondary`, etc. from Task 1
- Produces: charts that adapt colors to dark/light mode

- [ ] **Step 1: Update BarChart.svelte**

Replace hardcoded color values with CSS variable reads:

```svelte
<script lang="ts">
    import { onMount } from 'svelte';
    import Chart from 'chart.js/auto';

    let { labels = [], data = [], unit = 'hours' }: { labels?: string[]; data?: number[]; unit?: string } = $props();
    let canvas: HTMLCanvasElement;
    let chart: Chart;

    function getChartColor(varName: string): string {
        return getComputedStyle(document.documentElement).getPropertyValue(varName).trim();
    }

    function buildChart() {
        if (chart) chart.destroy();
        const primary = getChartColor('--chart-primary');
        const neutral = getChartColor('--chart-neutral');
        const text = getChartColor('--chart-text');

        chart = new Chart(canvas, {
            type: 'bar',
            data: {
                labels,
                datasets: [{
                    data,
                    backgroundColor: primary + '99',
                    borderColor: primary,
                    borderWidth: 1,
                    borderRadius: 6,
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: { legend: { display: false } },
                scales: {
                    y: {
                        beginAtZero: true,
                        grid: { color: neutral + '40' },
                        ticks: { color: text, font: { family: 'Inter' } },
                    },
                    x: {
                        grid: { display: false },
                        ticks: { color: text, font: { family: 'Inter' } },
                    }
                }
            }
        });
    }

    onMount(() => {
        buildChart();
        // Rebuild on theme change
        const observer = new MutationObserver(() => buildChart());
        observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] });
    });
</script>

<div class="w-full h-full min-h-[200px]">
    <canvas bind:this={canvas}></canvas>
</div>
```

- [ ] **Step 2: Update DonutChart.svelte**

Same pattern — read CSS variables for colors, rebuild on theme change.

- [ ] **Step 3: Update CategoryDonut.svelte**

Replace hardcoded `#` colors with CSS variable reads.

- [ ] **Step 4: Update ProductivityChart.svelte**

Replace hardcoded colors with CSS variable reads.

- [ ] **Step 5: Verify charts adapt to dark mode**

Toggle theme, verify all charts update colors.

- [ ] **Step 6: Commit**

```bash
git add src/lib/components/BarChart.svelte src/lib/components/DonutChart.svelte src/lib/components/CategoryDonut.svelte src/lib/components/ProductivityChart.svelte
git commit -m "feat: charts use CSS variables, adapt to dark/light theme"
```

---

### Task 6: TopBar Buttons — Calendar, Filter, Export, Add Limit

**Files:**
- Create: `src/lib/components/DatePickerPopover.svelte`
- Create: `src/lib/components/FilterPopover.svelte`
- Create: `src/lib/components/ExportDropdown.svelte`
- Create: `src/lib/components/QuickAddLimitModal.svelte`
- Modify: `src/lib/components/TopBar.svelte`

**Interfaces:**
- Consumes: `theme` store, `selectedRange` store, blocked apps store, activities store
- Produces: functional TopBar with 4 working buttons

- [ ] **Step 1: Create DatePickerPopover.svelte**

```svelte
<script lang="ts">
    let { onselect, onclose }: { onselect: (date: string) => void; onclose: () => void } = $props();
    let selectedDate = $state(new Date().toISOString().split('T')[0]);

    function handleSelect() {
        onselect(selectedDate);
        onclose();
    }
</script>

<div class="fixed inset-0 z-50" onclick={onclose} onkeydown={(e) => e.key === 'Escape' && onclose()}>
    <div class="absolute top-[72px] right-[200px] bg-surface-container-high border border-outline-variant/30 rounded-xl shadow-lg p-md w-[280px]" onclick={(e) => e.stopPropagation()}>
        <input
            type="date"
            bind:value={selectedDate}
            class="w-full bg-surface-container-low border border-outline-variant/30 rounded-lg px-md py-sm font-body-md text-on-surface focus:border-primary focus:ring-2 focus:ring-primary/20 outline-none"
        />
        <div class="flex justify-end gap-sm mt-md">
            <button class="px-md py-sm rounded-lg font-label-md text-on-surface-variant hover:bg-surface-container-low" onclick={onclose}>Cancel</button>
            <button class="px-md py-sm rounded-lg font-label-md text-on-primary bg-primary hover:bg-primary-container" onclick={handleSelect}>Go</button>
        </div>
    </div>
</div>
```

- [ ] **Step 2: Create FilterPopover.svelte**

```svelte
<script lang="ts">
    import { writable } from 'svelte/store';

    let { onapply, onclose }: { onapply: (filters: string[]) => void; onclose: () => void } = $props();

    const categories = ['Coding', 'Design', 'Communication', 'Entertainment', 'Neutral'];
    let selected = $state<string[]>([...categories]);

    function toggle(cat: string) {
        if (selected.includes(cat)) {
            selected = selected.filter(c => c !== cat);
        } else {
            selected = [...selected, cat];
        }
    }
</script>

<div class="fixed inset-0 z-50" onclick={onclose} onkeydown={(e) => e.key === 'Escape' && onclose()}>
    <div class="absolute top-[72px] right-[240px] bg-surface-container-high border border-outline-variant/30 rounded-xl shadow-lg p-md w-[220px]" onclick={(e) => e.stopPropagation()}>
        <p class="font-label-md text-on-surface font-semibold mb-sm">Filter by Category</p>
        {#each categories as cat}
            <label class="flex items-center gap-sm py-xs cursor-pointer hover:bg-surface-container-low rounded px-sm">
                <input type="checkbox" checked={selected.includes(cat)} onchange={() => toggle(cat)} class="accent-primary" />
                <span class="font-body-md text-on-surface">{cat}</span>
            </label>
        {/each}
        <div class="flex justify-end gap-sm mt-md">
            <button class="px-md py-sm rounded-lg font-label-md text-on-surface-variant hover:bg-surface-container-low" onclick={onclose}>Cancel</button>
            <button class="px-md py-sm rounded-lg font-label-md text-on-primary bg-primary hover:bg-primary-container" onclick={() => { onapply(selected); onclose(); }}>Apply</button>
        </div>
    </div>
</div>
```

- [ ] **Step 3: Create ExportDropdown.svelte**

```svelte
<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';

    let { onclose }: { onclose: () => void } = $props();

    async function exportCSV() {
        try {
            const data = await invoke<string>('export_activities_csv');
            const blob = new Blob([data], { type: 'text/csv' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `screentime-export-${new Date().toISOString().split('T')[0]}.csv`;
            a.click();
            URL.revokeObjectURL(url);
        } catch (e) {
            console.error('Export failed:', e);
        }
        onclose();
    }

    async function exportJSON() {
        try {
            const data = await invoke<string>('export_activities_json');
            const blob = new Blob([data], { type: 'application/json' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `screentime-export-${new Date().toISOString().split('T')[0]}.json`;
            a.click();
            URL.revokeObjectURL(url);
        } catch (e) {
            console.error('Export failed:', e);
        }
        onclose();
    }
</script>

<div class="fixed inset-0 z-50" onclick={onclose} onkeydown={(e) => e.key === 'Escape' && onclose()}>
    <div class="absolute top-[72px] right-[160px] bg-surface-container-high border border-outline-variant/30 rounded-xl shadow-lg p-sm w-[180px]" onclick={(e) => e.stopPropagation()}>
        <button class="w-full text-left px-md py-sm rounded-lg font-body-md text-on-surface hover:bg-surface-container-low flex items-center gap-sm" onclick={exportCSV}>
            <span class="material-symbols-outlined text-[18px]">description</span>
            Export CSV
        </button>
        <button class="w-full text-left px-md py-sm rounded-lg font-body-md text-on-surface hover:bg-surface-container-low flex items-center gap-sm" onclick={exportJSON}>
            <span class="material-symbols-outlined text-[18px]">data_object</span>
            Export JSON
        </button>
    </div>
</div>
```

- [ ] **Step 4: Create QuickAddLimitModal.svelte**

```svelte
<script lang="ts">
    import { blockedApps, fetchBlockedApps, updateAppLimits } from '$lib/stores/blockedApps';
    import { onMount } from 'svelte';

    let { open, onclose }: { open: boolean; onclose: () => void } = $props();
    let selectedApp = $state('');
    let dailyLimit = $state(60);
    let weeklyLimit = $state(300);

    onMount(() => fetchBlockedApps());

    function save() {
        const app = $blockedApps.find(a => a.app_name === selectedApp);
        if (app) {
            updateAppLimits(app.id, dailyLimit, weeklyLimit, true);
        }
        onclose();
    }
</script>

{#if open}
    <div class="fixed inset-0 z-50 bg-black/40 flex items-center justify-center" onclick={onclose} onkeydown={(e) => e.key === 'Escape' && onclose()}>
        <div class="bg-surface-container-high border border-outline-variant/30 rounded-2xl shadow-xl p-xl w-[400px]" onclick={(e) => e.stopPropagation()}>
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">Add Time Limit</h2>

            <label class="block mb-md">
                <span class="font-label-md text-on-surface-variant mb-xs block">App</span>
                <select bind:value={selectedApp} class="w-full bg-surface-container-low border border-outline-variant/30 rounded-lg px-md py-sm font-body-md text-on-surface">
                    <option value="">Select an app...</option>
                    {#each $blockedApps as app}
                        <option value={app.app_name}>{app.app_name}</option>
                    {/each}
                </select>
            </label>

            <label class="block mb-md">
                <span class="font-label-md text-on-surface-variant mb-xs block">Daily Limit (minutes)</span>
                <input type="number" bind:value={dailyLimit} min="0" class="w-full bg-surface-container-low border border-outline-variant/30 rounded-lg px-md py-sm font-body-md text-on-surface" />
            </label>

            <label class="block mb-lg">
                <span class="font-label-md text-on-surface-variant mb-xs block">Weekly Limit (minutes)</span>
                <input type="number" bind:value={weeklyLimit} min="0" class="w-full bg-surface-container-low border border-outline-variant/30 rounded-lg px-md py-sm font-body-md text-on-surface" />
            </label>

            <div class="flex justify-end gap-sm">
                <button class="px-md py-sm rounded-lg font-label-md text-on-surface-variant hover:bg-surface-container-low" onclick={onclose}>Cancel</button>
                <button class="px-md py-sm rounded-lg font-label-md text-on-primary bg-primary hover:bg-primary-container" onclick={save} disabled={!selectedApp}>Save</button>
            </div>
        </div>
    </div>
{/if}
```

- [ ] **Step 5: Update TopBar.svelte to wire up buttons**

```svelte
<script lang="ts">
    import ThemeToggle from './ThemeToggle.svelte';
    import DatePickerPopover from './DatePickerPopover.svelte';
    import FilterPopover from './FilterPopover.svelte';
    import ExportDropdown from './ExportDropdown.svelte';
    import QuickAddLimitModal from './QuickAddLimitModal.svelte';

    let { title = "Screen Time", subtitle = "" }: { title?: string; subtitle?: string } = $props();

    let showCalendar = $state(false);
    let showFilter = $state(false);
    let showExport = $state(false);
    let showAddLimit = $state(false);

    function handleDateSelect(date: string) {
        console.log('Selected date:', date);
        // TODO: integrate with timeRange store
    }

    function handleFilterApply(filters: string[]) {
        console.log('Active filters:', filters);
        // TODO: integrate with filter store
    }
</script>

<header class="fixed top-0 right-0 left-[280px] z-40 bg-surface/60 backdrop-blur-2xl border-b border-outline-variant/20 shadow-none flex items-center justify-between px-margin-desktop py-lg w-[calc(100%-280px)] h-[88px]">
    <div class="flex items-center gap-md">
        <h1 class="font-headline-md text-headline-md font-bold text-on-surface">{title}</h1>
        {#if subtitle}
            <span class="font-label-sm text-label-sm text-on-surface-variant px-sm py-xs bg-surface-container-low rounded-full">{subtitle}</span>
        {/if}
    </div>
    <div class="flex items-center gap-md">
        <div class="flex items-center gap-sm">
            <ThemeToggle />
            <button class="text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low p-sm rounded-full transition-colors flex items-center justify-center" onclick={() => showCalendar = !showCalendar}>
                <span class="material-symbols-outlined">calendar_today</span>
            </button>
            <button class="text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low p-sm rounded-full transition-colors flex items-center justify-center" onclick={() => showFilter = !showFilter}>
                <span class="material-symbols-outlined">tune</span>
            </button>
        </div>
        <div class="relative">
            <button class="font-label-md text-label-md text-primary bg-primary/5 hover:bg-primary/10 px-md py-sm rounded-lg transition-colors flex items-center gap-xs" onclick={() => showExport = !showExport}>
                <span class="material-symbols-outlined text-[18px]">share</span>
                Share
            </button>
            {#if showExport}
                <ExportDropdown onclose={() => showExport = false} />
            {/if}
        </div>
        <button class="font-label-md text-label-md text-on-primary bg-primary hover:bg-surface-tint px-md py-sm rounded-lg shadow-sm transition-all" onclick={() => showAddLimit = true}>
            Add Limit
        </button>
    </div>
</header>

{#if showCalendar}
    <DatePickerPopover onselect={handleDateSelect} onclose={() => showCalendar = false} />
{/if}

{#if showFilter}
    <FilterPopover onapply={handleFilterApply} onclose={() => showFilter = false} />
{/if}

<QuickAddLimitModal open={showAddLimit} onclose={() => showAddLimit = false} />
```

- [ ] **Step 6: Verify all buttons work**

Run dev server, test each button:
- Calendar: opens date picker, selecting date logs to console
- Tune: opens filter popover, checkboxes work, Apply logs filters
- Share: opens dropdown, CSV/JSON export triggers download
- Add Limit: opens modal, select app, set limits, save

- [ ] **Step 7: Commit**

```bash
git add src/lib/components/TopBar.svelte src/lib/components/DatePickerPopover.svelte src/lib/components/FilterPopover.svelte src/lib/components/ExportDropdown.svelte src/lib/components/QuickAddLimitModal.svelte
git commit -m "feat: TopBar buttons fully functional — calendar, filter, export, quick-add limit"
```

---

### Task 7: Settings Backend — Rust Commands

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Consumes: existing `settings` table (key TEXT, value TEXT)
- Produces: Tauri commands `get_settings`, `update_setting`, `export_activities_csv`, `export_activities_json`, `clear_all_data`, `reset_demo_data`

- [ ] **Step 1: Add Settings struct and commands to lib.rs**

Add before the `run()` function:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsResponse {
    pub settings: std::collections::HashMap<String, String>,
}

#[tauri::command]
fn get_settings(state: State<'_, AppState>) -> Result<SettingsResponse, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let mut stmt = conn.prepare("SELECT key, value FROM settings")
            .map_err(|e| e.to_string())?;
        let mut settings = std::collections::HashMap::new();
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).map_err(|e| e.to_string())?;
        for row in rows.flatten() {
            settings.insert(row.0, row.1);
        }
        Ok(SettingsResponse { settings })
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn update_setting(state: State<'_, AppState>, key: String, value: String) -> Result<(), String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        ).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn export_activities_csv(state: State<'_, AppState>) -> Result<String, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let mut stmt = conn.prepare("SELECT app_name, title, start_time, end_time, duration, category, productivity_score FROM activities")
            .map_err(|e| e.to_string())?;
        let mut csv = String::from("app_name,title,start_time,end_time,duration,category,productivity_score\n");
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, i64>(6)?,
            ))
        }).map_err(|e| e.to_string())?;
        for row in rows.flatten() {
            csv.push_str(&format!("{},{},{},{},{},{},{}\n", row.0, row.1, row.2, row.3, row.4, row.5, row.6));
        }
        Ok(csv)
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn export_activities_json(state: State<'_, AppState>) -> Result<String, String> {
    let activities = get_activities(state)?;
    serde_json::to_string_pretty(&activities).map_err(|e| e.to_string())
}

#[tauri::command]
fn clear_all_data(state: State<'_, AppState>) -> Result<(), String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        conn.execute("DELETE FROM activities", []).map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM blocked_apps", []).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn reset_demo_data(state: State<'_, AppState>) -> Result<(), String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        conn.execute("DELETE FROM activities", []).map_err(|e| e.to_string())?;
        seed_database(conn).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Database not initialized".to_string())
    }
}
```

- [ ] **Step 2: Register new commands in invoke_handler**

In the `run()` function, add to `tauri::generate_handler![]`:

```rust
.invoke_handler(tauri::generate_handler![
    get_activities,
    get_daily_summary,
    get_productivity_by_week,
    get_deep_work_sessions,
    get_blocked_apps,
    add_blocked_app,
    remove_blocked_app,
    toggle_blocked_app,
    update_app_limits,
    get_app_daily_usage,
    get_app_weekly_usage,
    get_settings,
    update_setting,
    export_activities_csv,
    export_activities_json,
    clear_all_data,
    reset_demo_data,
])
```

- [ ] **Step 3: Verify Rust compiles**

Run: `cd screen-time-app/src-tauri && cargo check`

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat: add settings, export, clear, and reset Tauri commands"
```

---

### Task 8: Settings Store + Settings Page

**Files:**
- Create: `src/lib/stores/settings.ts`
- Create: `src/routes/settings/+page.svelte`

**Interfaces:**
- Consumes: Tauri commands from Task 7, `theme` store from Task 2
- Produces: settings page UI, persistent settings

- [ ] **Step 1: Create settings store**

```typescript
import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { browser } from '$app/environment';

interface Settings {
    theme: 'system' | 'light' | 'dark';
    idle_timeout: string;
    tracking_paused: string;
    limit_warnings: string;
    daily_summary: string;
}

const defaults: Settings = {
    theme: 'system',
    idle_timeout: '5',
    tracking_paused: 'false',
    limit_warnings: 'true',
    daily_summary: 'true',
};

function createSettingsStore() {
    const { subscribe, set, update } = writable<Settings>({ ...defaults });
    let loaded = false;

    return {
        subscribe,
        load: async () => {
            if (!browser || loaded) return;
            try {
                const response = await invoke<{ settings: Record<string, string> }>('get_settings');
                const merged = { ...defaults, ...response.settings };
                set(merged);
                loaded = true;
            } catch (e) {
                console.error('Failed to load settings:', e);
            }
        },
        update: async (key: keyof Settings, value: string) => {
            update(s => ({ ...s, [key]: value }));
            try {
                await invoke('update_setting', { key, value });
            } catch (e) {
                console.error('Failed to save setting:', e);
            }
        },
        reset: () => {
            set({ ...defaults });
            loaded = false;
        }
    };
}

export const settings = createSettingsStore();
```

- [ ] **Step 2: Create settings page**

```svelte
<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
    import { settings } from '$lib/stores/settings';
    import { theme } from '$lib/stores/theme';
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';

    onMount(() => settings.load());

    function handleThemeChange(value: string) {
        settings.update('theme', value);
        theme.set(value as 'system' | 'light' | 'dark');
    }

    async function exportAllData() {
        try {
            const csv = await invoke<string>('export_activities_csv');
            const blob = new Blob([csv], { type: 'text/csv' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `screentime-full-export-${new Date().toISOString().split('T')[0]}.csv`;
            a.click();
            URL.revokeObjectURL(url);
        } catch (e) {
            console.error('Export failed:', e);
        }
    }

    async function clearAllData() {
        if (confirm('Are you sure? This will delete all tracked activities.')) {
            await invoke('clear_all_data');
            location.reload();
        }
    }

    async function resetDemoData() {
        if (confirm('Reset to demo data? This will replace all current data.')) {
            await invoke('reset_demo_data');
            location.reload();
        }
    }
</script>

<TopBar title="Settings" />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-3xl mx-auto w-full">
    <div class="space-y-xl">
        <!-- Appearance -->
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">Appearance</h2>

            <label class="block mb-md">
                <span class="font-label-md text-on-surface-variant mb-xs block">Theme</span>
                <div class="flex gap-sm">
                    {#each ['system', 'light', 'dark'] as option}
                        <button
                            class="px-md py-sm rounded-lg font-label-md transition-colors {$settings.theme === option ? 'bg-primary text-on-primary' : 'bg-surface-container-low text-on-surface-variant hover:bg-surface-container'}"
                            onclick={() => handleThemeChange(option)}
                        >
                            {option.charAt(0).toUpperCase() + option.slice(1)}
                        </button>
                    {/each}
                </div>
            </label>
        </section>

        <!-- Tracker -->
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">Tracker</h2>

            <label class="block mb-md">
                <span class="font-label-md text-on-surface-variant mb-xs block">Idle Detection Timeout</span>
                <select
                    value={$settings.idle_timeout}
                    onchange={(e) => settings.update('idle_timeout', (e.target as HTMLSelectElement).value)}
                    class="bg-surface-container-low border border-outline-variant/30 rounded-lg px-md py-sm font-body-md text-on-surface"
                >
                    <option value="5">5 minutes</option>
                    <option value="10">10 minutes</option>
                    <option value="15">15 minutes</option>
                    <option value="30">30 minutes</option>
                </select>
            </label>

            <label class="flex items-center justify-between py-sm">
                <span class="font-body-md text-on-surface">Pause Tracking</span>
                <button
                    class="w-12 h-6 rounded-full transition-colors relative {$settings.tracking_paused === 'true' ? 'bg-primary' : 'bg-outline-variant'}"
                    onclick={() => settings.update('tracking_paused', $settings.tracking_paused === 'true' ? 'false' : 'true')}
                >
                    <span class="absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform {$settings.tracking_paused === 'true' ? 'translate-x-6' : ''}"></span>
                </button>
            </label>
        </section>

        <!-- Notifications -->
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">Notifications</h2>

            <label class="flex items-center justify-between py-sm">
                <span class="font-body-md text-on-surface">Limit Warnings</span>
                <button
                    class="w-12 h-6 rounded-full transition-colors relative {$settings.limit_warnings === 'true' ? 'bg-primary' : 'bg-outline-variant'}"
                    onclick={() => settings.update('limit_warnings', $settings.limit_warnings === 'true' ? 'false' : 'true')}
                >
                    <span class="absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform {$settings.limit_warnings === 'true' ? 'translate-x-6' : ''}"></span>
                </button>
            </label>

            <label class="flex items-center justify-between py-sm">
                <span class="font-body-md text-on-surface">Daily Summary</span>
                <button
                    class="w-12 h-6 rounded-full transition-colors relative {$settings.daily_summary === 'true' ? 'bg-primary' : 'bg-outline-variant'}"
                    onclick={() => settings.update('daily_summary', $settings.daily_summary === 'true' ? 'false' : 'true')}
                >
                    <span class="absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform {$settings.daily_summary === 'true' ? 'translate-x-6' : ''}"></span>
                </button>
            </label>
        </section>

        <!-- Data Management -->
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">Data Management</h2>

            <div class="flex flex-col gap-sm">
                <button class="bg-primary/5 text-primary hover:bg-primary/10 px-md py-sm rounded-lg font-label-md text-label-md transition-colors text-left flex items-center gap-sm" onclick={exportAllData}>
                    <span class="material-symbols-outlined text-[18px]">download</span>
                    Export All Data (CSV)
                </button>
                <button class="bg-error/5 text-error hover:bg-error/10 px-md py-sm rounded-lg font-label-md text-label-md transition-colors text-left flex items-center gap-sm" onclick={clearAllData}>
                    <span class="material-symbols-outlined text-[18px]">delete</span>
                    Clear All Data
                </button>
                <button class="bg-surface-container-low text-on-surface-variant hover:bg-surface-container px-md py-sm rounded-lg font-label-md text-label-md transition-colors text-left flex items-center gap-sm" onclick={resetDemoData}>
                    <span class="material-symbols-outlined text-[18px]">refresh</span>
                    Reset Demo Data
                </button>
            </div>
        </section>

        <!-- About -->
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">About</h2>
            <div class="space-y-sm font-body-md text-on-surface-variant">
                <p><strong class="text-on-surface">Screen Time Tracker</strong> v1.0.0</p>
                <p>Track your digital wellness. All data stays local on your device.</p>
            </div>
        </section>
    </div>
</main>
```

- [ ] **Step 3: Verify settings page renders**

Run dev server, navigate to `/settings` — verify all sections render, toggles work, theme changes persist.

- [ ] **Step 4: Commit**

```bash
git add src/lib/stores/settings.ts src/routes/settings/+page.svelte
git commit -m "feat: settings page with appearance, tracker, notifications, data management"
```

---

### Task 9: Help Page

**Files:**
- Create: `src/routes/help/+page.svelte`

**Interfaces:**
- Consumes: none
- Produces: help page UI

- [ ] **Step 1: Create help page**

```svelte
<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
</script>

<TopBar title="Help" />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-3xl mx-auto w-full">
    <div class="space-y-xl">
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-md">How Tracking Works</h2>
            <div class="font-body-md text-on-surface-variant space-y-sm">
                <p>Screen Time Tracker monitors your active window every 3 seconds to understand how you spend your time on the computer.</p>
                <p>Apps are automatically categorized as Coding, Design, Communication, Entertainment, or Neutral. Browser tabs are tracked by site name.</p>
            </div>
        </section>

        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-md">Blocked Apps & Limits</h2>
            <div class="font-body-md text-on-surface-variant space-y-sm">
                <p>You can block specific apps or set daily/weekly time limits. When a limit is reached, the app will be closed automatically.</p>
                <p>Go to <strong class="text-on-surface">Blocked Apps</strong> in the sidebar to manage restrictions.</p>
            </div>
        </section>

        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-md">Data Privacy</h2>
            <div class="font-body-md text-on-surface-variant space-y-sm">
                <p>All data is stored locally in an SQLite database on your device. Nothing is sent to the cloud.</p>
                <p>You can export or delete all data at any time from <strong class="text-on-surface">Settings</strong>.</p>
            </div>
        </section>

        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-md">Support</h2>
            <div class="font-body-md text-on-surface-variant space-y-sm">
                <p>Found a bug or have a feature request?</p>
                <a href="https://github.com" class="text-primary hover:underline inline-flex items-center gap-xs">
                    <span class="material-symbols-outlined text-[18px]">open_in_new</span>
                    Open an issue on GitHub
                </a>
            </div>
        </section>
    </div>
</main>
```

- [ ] **Step 2: Verify help page renders**

Run dev server, navigate to `/help` — verify all sections display correctly.

- [ ] **Step 3: Commit**

```bash
git add src/routes/help/+page.svelte
git commit -m "feat: help page with FAQ, privacy info, and support links"
```

---

### Task 10: Sidebar — Activate Settings/Help Links

**Files:**
- Modify: `src/lib/components/Sidebar.svelte`

**Interfaces:**
- Consumes: `$page.url.pathname` from SvelteKit
- Produces: active states for Settings and Help routes

- [ ] **Step 1: Update Sidebar.svelte**

Change the Settings link from `href="#"` to `href="/settings"` and add active state:

```svelte
<a class="{$page.url.pathname.includes('/settings') ? 'bg-primary/10 text-primary font-bold' : 'text-on-surface-variant hover:bg-surface-container-low'} rounded-lg px-md py-sm flex items-center gap-sm transition-colors duration-200" href="/settings">
    <span class="material-symbols-outlined" style={$page.url.pathname.includes('/settings') ? "font-variation-settings: 'FILL' 1;" : ""}>settings</span>
    <span class="font-label-md text-label-md">Settings</span>
</a>
```

Change the Help link from `href="#"` to `href="/help"` and add active state:

```svelte
<a class="{$page.url.pathname.includes('/help') ? 'bg-primary/10 text-primary font-bold' : 'text-on-surface-variant hover:bg-surface-container-low'} rounded-lg px-md py-sm flex items-center gap-sm transition-colors duration-200" href="/help">
    <span class="material-symbols-outlined" style={$page.url.pathname.includes('/help') ? "font-variation-settings: 'FILL' 1;" : ""}>help</span>
    <span class="font-label-md text-label-md">Help</span>
</a>
```

- [ ] **Step 2: Verify sidebar navigation**

Run dev server, click Settings and Help in sidebar — verify they navigate to correct pages and show active state.

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/Sidebar.svelte
git commit -m "feat: sidebar Settings and Help links active with route-based highlighting"
```

---

## Verification Checklist

After all tasks are complete:

1. Toggle dark mode on every page — all elements should use correct dark palette
2. Charts should adapt colors when switching themes
3. ThemeToggle cycles System → Light → Dark
4. All 4 TopBar buttons open their respective UIs and perform actions
5. Settings page loads, all toggles/dropdowns work, changes persist
6. Help page renders all sections
7. Sidebar links navigate correctly with active state highlighting
8. Run `npm run check` — no TypeScript errors
9. Run `npm run build` — production build succeeds
