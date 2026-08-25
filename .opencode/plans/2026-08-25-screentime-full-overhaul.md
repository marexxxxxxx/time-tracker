# Screen Time Dashboard — Full Overhaul Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fully functional screen-time dashboard with real data, charting, dark mode, and blocked-apps system.

**Architecture:** Bottom-up: extract components from monolithic pages, add dark mode toggle, connect backend data, replace decorative charts with Chart.js, and build a blocked-apps enforcement system. Each phase produces working, testable software.

**Tech Stack:** Tauri 2, Svelte 5 (runes), Tailwind CSS 3.4, SQLite (rusqlite), Rust, Chart.js + svelte-chartjs

**Spec:** `.opencode/plans/2026-08-25-screentime-full-overhaul-design.md`

## Global Constraints

- Linux (Hyprland/Wayland primary, X11 fallback)
- Tauri 2, Svelte 5 runes (`$props()`, `$state`, `$derived`), no Svelte 4 syntax
- M3 color tokens from `tailwind.config.js` — no hardcoded hex for component colors
- Dark mode: `darkMode: "class"` in Tailwind; toggle sets class on `<html>`; default: light
- All backend commands registered via `tauri::generate_handler![]` in `lib.rs`
- SPA mode (adapter-static, SSR disabled) — no server-side code
- Charts: `chart.js` v4 + `svelte-chartjs` v3
- Existing polling: layout polls `get_activities` every 5s; blocked app enforcement polls every 3s in tracker.rs

---

## Phase 1: Component Decomposition + Svelte 5 Consistency

### Task 1: Migrate TopBar to Svelte 5 Props

**Files:**
- Modify: `screen-time-app/src/lib/components/TopBar.svelte`

**Interfaces:**
- Produces: `TopBar` component with `title`, `subtitle` props via `$props()`

- [ ] **Step 1: Replace `export let` with `$props()`**

Replace the script block in `TopBar.svelte`:
```svelte
<script lang="ts">
    import ThemeToggle from './ThemeToggle.svelte';
    let { title = "Screen Time", subtitle = "" }: { title?: string; subtitle?: string } = $props();
</script>
```

- [ ] **Step 2: Add ThemeToggle slot in the header actions**

In the header's right-side button group, add `<ThemeToggle />` before the calendar/tune buttons:
```svelte
<div class="flex items-center gap-sm">
    <ThemeToggle />
    <button class="...">calendar</button>
    <button class="...">tune</button>
</div>
```

Note: ThemeToggle won't exist yet (created in Phase 2). For now, add the import and component tag. If it errors during Phase 1 testing, temporarily comment out the import.

- [ ] **Step 3: Verify Overview page renders**

Run: `npm run check` (in `screen-time-app/`)
Expected: No TypeScript errors related to TopBar props

- [ ] **Step 4: Commit**

```bash
git add screen-time-app/src/lib/components/TopBar.svelte
git commit -m "refactor: migrate TopBar to Svelte 5 props"
```

---

### Task 2: Create StatCard Component

**Files:**
- Create: `screen-time-app/src/lib/components/StatCard.svelte`

**Interfaces:**
- Consumes: `icon` (string), `label` (string), `value` (string), `subtext` (string, optional), `progress` (number, optional, 0-100), `progressColor` (string, optional, default `"bg-primary"`)

- [ ] **Step 1: Create `StatCard.svelte`**

```svelte
<script lang="ts">
    let {
        icon,
        label,
        value,
        subtext = "",
        progress = -1,
        progressColor = "bg-primary"
    }: {
        icon: string;
        label: string;
        value: string;
        subtext?: string;
        progress?: number;
        progressColor?: string;
    } = $props();
</script>

<div class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20 flex flex-col justify-between">
    <div>
        <div class="flex items-center gap-sm mb-sm">
            <span class="material-symbols-outlined text-primary">{icon}</span>
            <h2 class="font-label-md text-label-md text-on-surface-variant">{label}</h2>
        </div>
        <div class="font-display text-display text-on-surface">{value}</div>
        {#if subtext}
            <div class="font-body-md text-body-md text-on-surface-variant flex items-center gap-xs mt-xs">
                {subtext}
            </div>
        {/if}
    </div>
    {#if progress >= 0}
        <div class="mt-xl">
            <div class="h-2 w-full bg-surface-container-high rounded-full overflow-hidden">
                <div class="h-full {progressColor} rounded-full" style="width: {progress}%"></div>
            </div>
        </div>
    {/if}
</div>
```

- [ ] **Step 2: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass (component has no external dependencies)

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/lib/components/StatCard.svelte
git commit -m "feat: add StatCard component"
```

---

### Task 3: Create AppUsageList Component

**Files:**
- Create: `screen-time-app/src/lib/components/AppUsageList.svelte`

**Interfaces:**
- Consumes: `items` array of `{ name: string, duration: number, color: string, icon: string }`, `totalDuration: number`
- Uses: `formatDuration()` from `$lib/stores/activities`

- [ ] **Step 1: Create `AppUsageList.svelte`**

```svelte
<script lang="ts">
    import { formatDuration } from '$lib/stores/activities';

    let { items, totalDuration }: {
        items: Array<{ name: string; duration: number; color: string; icon: string }>;
        totalDuration: number;
    } = $props();

    function pct(dur: number): number {
        if (totalDuration === 0) return 0;
        return Math.round((dur / totalDuration) * 100);
    }
</script>

<h2 class="font-headline-md text-headline-md text-on-surface mb-xl">Most Used</h2>
<div class="flex flex-col gap-lg">
    {#each items as item}
        <div>
            <div class="flex justify-between items-center mb-sm">
                <div class="flex items-center gap-sm">
                    <div class="w-8 h-8 rounded-lg flex items-center justify-center" style="background-color: {item.color}">
                        <span class="material-symbols-outlined text-white text-[18px]">{item.icon}</span>
                    </div>
                    <span class="font-body-md text-body-md text-on-surface">{item.name}</span>
                </div>
                <span class="font-label-md text-label-md text-on-surface-variant">{formatDuration(item.duration)}</span>
            </div>
            <div class="h-2 w-full bg-surface-container-high rounded-full overflow-hidden">
                <div class="h-full rounded-full" style="width: {pct(item.duration)}%; background-color: {item.color}"></div>
            </div>
        </div>
    {/each}
</div>
```

- [ ] **Step 2: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/lib/components/AppUsageList.svelte
git commit -m "feat: add AppUsageList component"
```

---

### Task 4: Create CategoryDonut Component

**Files:**
- Create: `screen-time-app/src/lib/components/CategoryDonut.svelte`

**Interfaces:**
- Consumes: `categories` array of `{ name: string, percentage: number, color: string }`

- [ ] **Step 1: Create `CategoryDonut.svelte`**

This is the placeholder SVG donut — will be replaced with Chart.js in Phase 4.

```svelte
<script lang="ts">
    let { categories }: {
        categories: Array<{ name: string; percentage: number; color: string }>;
    } = $props();

    let segments = $derived(
        categories.reduce((acc, cat, i) => {
            const offset = acc.length > 0 ? acc[acc.length - 1].offset + acc[acc.length - 1].pct : 0;
            acc.push({ ...cat, offset });
            return acc;
        }, [] as Array<{ name: string; percentage: number; color: string; offset: number }>)
    );
</script>

<h2 class="font-headline-md text-headline-md text-on-surface mb-xl">Categories</h2>
<div class="flex gap-lg items-center">
    <div class="relative w-32 h-32 flex-shrink-0">
        <svg class="w-full h-full transform -rotate-90" viewBox="0 0 36 36">
            <path class="text-surface-container-high" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" stroke-width="4"></path>
            {#each segments as seg}
                <path
                    d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                    fill="none"
                    stroke={seg.color}
                    stroke-dasharray="{seg.percentage}, 100"
                    stroke-dashoffset="-{seg.offset}"
                    stroke-width="4"
                ></path>
            {/each}
        </svg>
    </div>
    <div class="flex-1 flex flex-col gap-md">
        {#each categories as cat}
            <div class="flex justify-between items-center">
                <div class="flex items-center gap-sm">
                    <div class="w-3 h-3 rounded-full" style="background-color: {cat.color}"></div>
                    <span class="font-body-md text-body-md text-on-surface">{cat.name}</span>
                </div>
                <span class="font-label-md text-label-md text-on-surface-variant">{cat.percentage}%</span>
            </div>
        {/each}
    </div>
</div>
```

- [ ] **Step 2: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/lib/components/CategoryDonut.svelte
git commit -m "feat: add CategoryDonut placeholder component"
```

---

### Task 5: Create PageHeader and TimeRangeSelector Components

**Files:**
- Create: `screen-time-app/src/lib/components/PageHeader.svelte`
- Create: `screen-time-app/src/lib/components/TimeRangeSelector.svelte`

**Interfaces:**
- `PageHeader` consumes: `title` (string), `description` (string, optional)
- `TimeRangeSelector` consumes: `selected` (string), `onselect` callback

- [ ] **Step 1: Create `PageHeader.svelte`**

```svelte
<script lang="ts">
    let { title, description = "" }: { title: string; description?: string } = $props();
</script>

<div class="mb-lg">
    <h2 class="font-headline-lg text-headline-lg text-on-surface mb-xs font-semibold">{title}</h2>
    {#if description}
        <p class="font-body-md text-body-md text-on-surface-variant">{description}</p>
    {/if}
</div>
```

- [ ] **Step 2: Create `TimeRangeSelector.svelte`**

```svelte
<script lang="ts">
    let { selected = "Day", options = ["Day", "Week", "Month"] }: {
        selected?: string;
        options?: string[];
    } = $props();

    function select(opt: string) {
        selected = opt;
    }
</script>

<div class="flex bg-surface-container-low p-xs rounded-lg">
    {#each options as opt}
        <button
            class="px-md py-xs font-label-md text-label-md rounded-md transition-colors {selected === opt ? 'bg-surface-container-lowest shadow-sm text-on-surface' : 'text-on-surface-variant hover:text-on-surface'}"
            onclick={() => select(opt)}
        >
            {opt}
        </button>
    {/each}
</div>
```

- [ ] **Step 3: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 4: Commit**

```bash
git add screen-time-app/src/lib/components/PageHeader.svelte screen-time-app/src/lib/components/TimeRangeSelector.svelte
git commit -m "feat: add PageHeader and TimeRangeSelector components"
```

---

### Task 6: Rewrite Overview Page with Extracted Components

**Files:**
- Modify: `screen-time-app/src/routes/+page.svelte`

**Interfaces:**
- Consumes: `StatCard`, `AppUsageList`, `CategoryDonut`, `TimeRangeSelector`
- Still uses hardcoded data (backend connection in Phase 3)

- [ ] **Step 1: Rewrite Overview page**

Replace the full content of `+page.svelte` with:

```svelte
<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
    import StatCard from '$lib/components/StatCard.svelte';
    import AppUsageList from '$lib/components/AppUsageList.svelte';
    import CategoryDonut from '$lib/components/CategoryDonut.svelte';
    import TimeRangeSelector from '$lib/components/TimeRangeSelector.svelte';

    const appItems = [
        { name: "VS Code", duration: 8100, color: "#000000", icon: "code" },
        { name: "Safari", duration: 6300, color: "#0070eb", icon: "explore" },
        { name: "Slack", duration: 3300, color: "#4A154B", icon: "forum" },
    ];
    const totalAppDuration = appItems.reduce((s, i) => s + i.duration, 0);

    const categories = [
        { name: "Productivity", percentage: 60, color: "#0058bc" },
        { name: "Communication", percentage: 25, color: "#006e28" },
        { name: "Entertainment", percentage: 15, color: "#4c4aca" },
    ];
</script>

<TopBar title="Overview" subtitle="Today" />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-7xl mx-auto w-full">
    <div class="grid grid-cols-12 gap-lg">
        <!-- Summary -->
        <div class="col-span-12 lg:col-span-4">
            <StatCard
                icon="schedule"
                label="Total Screen Time"
                value="6h 45m"
                subtext="12% from yesterday"
                progress={78}
            />
        </div>

        <!-- Chart placeholder -->
        <div class="col-span-12 lg:col-span-8 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20 flex flex-col">
            <div class="flex justify-between items-center mb-xl">
                <h2 class="font-headline-md text-headline-md text-on-surface">Daily Usage</h2>
                <TimeRangeSelector />
            </div>
            <div class="flex-1 flex items-end gap-md pt-lg min-h-[200px]">
                {#each [{label:"6AM",h:"30%"},{label:"9AM",h:"50%"},{label:"12PM",h:"90%",active:true},{label:"3PM",h:"70%"},{label:"6PM",h:"40%"},{label:"9PM",h:"20%"}] as bar}
                    <div class="flex-1 flex flex-col justify-end group">
                        <div class="w-full rounded-t-full transition-colors {bar.active ? 'bg-primary shadow-sm' : 'bg-primary/20 hover:bg-primary/40'}" style="height: {bar.h}"></div>
                        <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm {bar.active ? 'font-semibold' : ''}">{bar.label}</div>
                    </div>
                {/each}
            </div>
        </div>

        <!-- Most Used Apps -->
        <div class="col-span-12 lg:col-span-6 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20">
            <AppUsageList items={appItems} totalDuration={totalAppDuration} />
        </div>

        <!-- Categories -->
        <div class="col-span-12 lg:col-span-6 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20">
            <CategoryDonut categories={categories} />
        </div>
    </div>
</main>
```

- [ ] **Step 2: Verify with `npm run check` and `npm run build`**

Run: `npm run check && npm run build` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/routes/+page.svelte
git commit -m "refactor: rewrite Overview page with extracted components"
```

---

### Task 7: Create ProductivityChart and DeepWorkTimeline Components

**Files:**
- Create: `screen-time-app/src/lib/components/ProductivityChart.svelte`
- Create: `screen-time-app/src/lib/components/DeepWorkTimeline.svelte`

**Interfaces:**
- `ProductivityChart` consumes: `data` array of `{ day: string, productive: number, neutral: number, leisure: number }`
- `DeepWorkTimeline` consumes: `sessions` array of `{ title: string, startTime: string, endTime: string, duration: string, category: string, color: string }`

- [ ] **Step 1: Create `ProductivityChart.svelte`** (placeholder stacked bars)

```svelte
<script lang="ts">
    let { data }: {
        data: Array<{ day: string; productive: number; neutral: number; leisure: number }>;
    } = $props();
</script>

<h3 class="font-headline-md text-headline-md text-on-surface font-semibold mb-lg">Work vs. Leisure</h3>
<div class="flex items-end gap-sm h-48 mt-md border-b border-outline-variant/30 pb-sm">
    {#each data as d}
        <div class="flex-1 flex flex-col justify-end group">
            <div class="w-full bg-tertiary rounded-t-full transition-all opacity-80 group-hover:opacity-100" style="height: {d.leisure}%"></div>
            <div class="w-full bg-surface-variant transition-all opacity-80 group-hover:opacity-100" style="height: {d.neutral}%"></div>
            <div class="w-full bg-primary transition-all opacity-80 group-hover:opacity-100 rounded-b-sm" style="height: {d.productive}%"></div>
            <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm">{d.day}</div>
        </div>
    {/each}
</div>
```

- [ ] **Step 2: Create `DeepWorkTimeline.svelte`**

```svelte
<script lang="ts">
    let { sessions }: {
        sessions: Array<{
            title: string;
            startTime: string;
            endTime: string;
            duration: string;
            category: string;
            color: string;
        }>;
    } = $props();
</script>

<h3 class="font-headline-md text-headline-md text-on-surface font-semibold mb-lg">Recent Deep Work</h3>
<div class="space-y-sm">
    {#each sessions as session}
        <div class="flex items-center justify-between p-sm hover:bg-surface-container-low rounded-lg transition-colors border border-transparent hover:border-outline-variant/30">
            <div class="flex items-center gap-md">
                <div class="w-2 h-10 rounded-full" style="background-color: {session.color}"></div>
                <div>
                    <p class="font-body-md text-body-md font-medium text-on-surface">{session.title}</p>
                    <p class="font-label-sm text-label-sm text-on-surface-variant flex items-center gap-xs">
                        <span class="material-symbols-outlined text-[14px]">schedule</span>
                        {session.startTime} - {session.endTime}
                    </p>
                </div>
            </div>
            <div class="text-right">
                <p class="font-headline-md text-body-md font-semibold" style="color: {session.color}">{session.duration}</p>
                <span class="inline-block px-2 py-0.5 rounded-md bg-surface-variant text-on-surface-variant font-label-sm text-[10px] uppercase">{session.category}</span>
            </div>
        </div>
    {/each}
</div>
```

- [ ] **Step 3: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 4: Commit**

```bash
git add screen-time-app/src/lib/components/ProductivityChart.svelte screen-time-app/src/lib/components/DeepWorkTimeline.svelte
git commit -m "feat: add ProductivityChart and DeepWorkTimeline components"
```

---

### Task 8: Rewrite Productivity Page with Extracted Components

**Files:**
- Modify: `screen-time-app/src/routes/productivity/+page.svelte`

**Interfaces:**
- Consumes: `StatCard`, `ProductivityChart`, `DeepWorkTimeline`, `CategoryDonut`

- [ ] **Step 1: Rewrite Productivity page**

```svelte
<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
    import StatCard from '$lib/components/StatCard.svelte';
    import ProductivityChart from '$lib/components/ProductivityChart.svelte';
    import DeepWorkTimeline from '$lib/components/DeepWorkTimeline.svelte';
    import CategoryDonut from '$lib/components/CategoryDonut.svelte';

    const weeklyData = [
        { day: "Mon", productive: 60, neutral: 10, leisure: 20 },
        { day: "Tue", productive: 70, neutral: 5, leisure: 15 },
        { day: "Wed", productive: 40, neutral: 20, leisure: 30 },
        { day: "Thu", productive: 80, neutral: 10, leisure: 10 },
    ];

    const deepSessions = [
        { title: "VS Code Architecture", startTime: "09:00 AM", endTime: "11:30 AM", duration: "2h 30m", category: "Coding", color: "#0058bc" },
        { title: "Figma UI Kit Update", startTime: "01:00 PM", endTime: "02:20 PM", duration: "1h 20m", category: "Design", color: "#4c4aca" },
        { title: "Code Review & PRs", startTime: "03:00 PM", endTime: "04:15 PM", duration: "1h 15m", category: "Coding", color: "#0058bc" },
    ];

    const categories = [
        { name: "Coding", percentage: 65, color: "#0058bc" },
        { name: "Design", percentage: 25, color: "#4c4aca" },
        { name: "Writing", percentage: 10, color: "#006e28" },
    ];
</script>

<TopBar title="Productivity Tracker" />

<main class="flex-1 overflow-y-auto p-margin-desktop mt-[88px]">
    <div class="max-w-[1200px] mx-auto grid grid-cols-12 gap-lg pb-xxl">
        <div class="col-span-12 md:col-span-4">
            <StatCard
                icon="trending_up"
                label="Productivity Score"
                value="84%"
                subtext="+5% from yesterday"
                progress={84}
            />
        </div>

        <div class="col-span-12 md:col-span-8 bg-surface-container-lowest border border-outline-variant rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20">
            <ProductivityChart data={weeklyData} />
        </div>

        <div class="col-span-12 md:col-span-5 bg-surface-container-lowest border border-outline-variant rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20">
            <CategoryDonut categories={categories} />
        </div>

        <div class="col-span-12 md:col-span-7 bg-surface-container-lowest border border-outline-variant rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20">
            <DeepWorkTimeline sessions={deepSessions} />
        </div>
    </div>
</main>
```

- [ ] **Step 2: Verify with `npm run check && npm run build`**

Run: `npm run check && npm run build` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/routes/productivity/+page.svelte
git commit -m "refactor: rewrite Productivity page with extracted components"
```

---

### Task 9: Create AppBlockCard Component

**Files:**
- Create: `screen-time-app/src/lib/components/AppBlockCard.svelte`

**Interfaces:**
- Consumes: `appName` (string), `icon` (string), `iconBg` (string), `limit` (string), `isBlocked` (boolean), `onToggle` callback, `usage` (string, optional), `usagePct` (number, optional)

- [ ] **Step 1: Create `AppBlockCard.svelte`**

```svelte
<script lang="ts">
    let {
        appName,
        icon,
        iconBg,
        limit,
        isBlocked,
        onToggle,
        usage = "",
        usagePct = -1
    }: {
        appName: string;
        icon: string;
        iconBg: string;
        limit: string;
        isBlocked: boolean;
        onToggle: () => void;
        usage?: string;
        usagePct?: number;
    } = $props();
</script>

<div class="flex items-center justify-between p-lg hover:bg-surface-container-lowest transition-colors {isBlocked ? 'opacity-60' : ''}">
    <div class="flex items-center gap-md">
        <div class="w-10 h-10 rounded-lg flex items-center justify-center" style="background-color: {iconBg}">
            <span class="material-symbols-outlined">{icon}</span>
        </div>
        <div>
            <h4 class="font-body-md text-body-md font-medium text-on-surface">{appName}</h4>
            <p class="font-label-sm text-label-sm text-on-surface-variant">{limit}</p>
        </div>
    </div>
    <div class="flex items-center gap-md">
        {#if usagePct >= 0}
            <div class="bg-surface-variant h-1.5 rounded-full w-24 overflow-hidden mr-4">
                <div class="h-1.5 rounded-full" style="width: {usagePct}%; background-color: {iconBg}"></div>
            </div>
            <p class="font-label-sm text-label-sm text-on-surface-variant w-12 text-right mr-4">{usage}</p>
        {/if}
        {#if isBlocked}
            <p class="font-label-sm text-label-sm text-error w-auto mr-4 flex items-center gap-xs">
                <span class="material-symbols-outlined text-[14px]">lock</span>
                Blocked
            </p>
        {/if}
        <button
            class="relative inline-block w-12 align-middle select-none transition duration-200 ease-in"
            onclick={onToggle}
            aria-label="Toggle {appName}"
        >
            <div class="block overflow-hidden h-6 rounded-full transition-colors duration-200 ease-in-out {isBlocked ? 'bg-primary' : 'bg-surface-variant'}">
                <div class="absolute top-[2px] left-[2px] w-5 h-5 bg-white rounded-full shadow-sm transition-transform duration-200 ease-in-out {isBlocked ? 'translate-x-[24px]' : 'translate-x-0'}"></div>
            </div>
        </button>
    </div>
</div>
```

- [ ] **Step 2: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/lib/components/AppBlockCard.svelte
git commit -m "feat: add AppBlockCard component with toggle"
```

---

### Task 10: Rewrite Blocked Page with AppBlockCard

**Files:**
- Modify: `screen-time-app/src/routes/blocked/+page.svelte`

**Interfaces:**
- Consumes: `AppBlockCard`, `PageHeader`
- State: `$state` array of blocked app objects with local toggle handlers

- [ ] **Step 1: Rewrite Blocked page**

```svelte
<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
    import PageHeader from '$lib/components/PageHeader.svelte';
    import AppBlockCard from '$lib/components/AppBlockCard.svelte';

    let blockedApps = $state([
        { appName: "Instagram", icon: "photo_camera", iconBg: "rgba(225,48,108,0.1)", category: "Social Media", limit: "Limit: 30m / day", isBlocked: true, usage: "24m used", usagePct: 80 },
        { appName: "TikTok", icon: "music_note", iconBg: "rgba(0,0,0,0.05)", category: "Social Media", limit: "Blocked entirely", isBlocked: true },
        { appName: "Twitter", icon: "chat_bubble", iconBg: "rgba(29,161,242,0.1)", category: "Social Media", limit: "Limit: 1h / day", isBlocked: false, usage: "6m used", usagePct: 10 },
        { appName: "YouTube", icon: "play_arrow", iconBg: "rgba(255,0,0,0.1)", category: "Entertainment", limit: "Limit: 2h / day", isBlocked: true },
        { appName: "Netflix", icon: "movie", iconBg: "rgba(229,9,20,0.1)", category: "Entertainment", limit: "Limit: 1h 30m / day", isBlocked: true },
    ]);

    function toggleApp(index: number) {
        blockedApps[index].isBlocked = !blockedApps[index].isBlocked;
    }

    function appsForCategory(cat: string) {
        return blockedApps
            .map((a, i) => ({ ...a, index: i }))
            .filter(a => a.category === cat);
    }

    const socialApps = $derived(appsForCategory("Social Media"));
    const entertainmentApps = $derived(appsForCategory("Entertainment"));
</script>

<TopBar title="Screen Time" />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-7xl mx-auto w-full">
    <div class="max-w-4xl mx-auto space-y-xl">
        <PageHeader title="Blocked Apps & Limits" description="Manage restrictions and focus schedules for your applications." />

        <!-- Social Media -->
        <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
            <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Social Media</h3>
                <span class="font-label-sm text-label-sm text-on-surface-variant bg-surface-variant/50 px-sm py-xs rounded-md">{socialApps.length} Apps</span>
            </div>
            <div class="divide-y divide-outline-variant/20">
                {#each socialApps as app}
                    <AppBlockCard
                        appName={app.appName}
                        icon={app.icon}
                        iconBg={app.iconBg}
                        limit={app.limit}
                        isBlocked={app.isBlocked}
                        usage={app.usage}
                        usagePct={app.usagePct}
                        onToggle={() => toggleApp(app.index)}
                    />
                {/each}
            </div>
        </section>

        <!-- Entertainment -->
        <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
            <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Entertainment</h3>
                <span class="font-label-sm text-label-sm text-on-surface-variant bg-surface-variant/50 px-sm py-xs rounded-md">{entertainmentApps.length} Apps</span>
            </div>
            <div class="divide-y divide-outline-variant/20">
                {#each entertainmentApps as app}
                    <AppBlockCard
                        appName={app.appName}
                        icon={app.icon}
                        iconBg={app.iconBg}
                        limit={app.limit}
                        isBlocked={app.isBlocked}
                        usage={app.usage}
                        usagePct={app.usagePct}
                        onToggle={() => toggleApp(app.index)}
                    />
                {/each}
            </div>
        </section>
    </div>
</main>
```

- [ ] **Step 2: Verify with `npm run check && npm run build`**

Run: `npm run check && npm run build` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/routes/blocked/+page.svelte
git commit -m "refactor: rewrite Blocked page with AppBlockCard and state"
```

---

## Phase 2: Dark Mode Toggle

### Task 11: Create Theme Store

**Files:**
- Create: `screen-time-app/src/lib/stores/theme.ts`

**Interfaces:**
- Produces: `theme` writable store (`"light" | "dark"`), `toggleTheme()` function
- Reads/writes `localStorage`, syncs `document.documentElement.classList`

- [ ] **Step 1: Create `theme.ts`**

```typescript
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

function createThemeStore() {
    const stored = browser ? localStorage.getItem('theme') as 'light' | 'dark' | null : null;
    const initial = stored ?? 'light';
    const { subscribe, set, update } = writable<'light' | 'dark'>(initial);

    if (browser) {
        document.documentElement.classList.toggle('dark', initial === 'dark');
    }

    return {
        subscribe,
        toggle: () => {
            update(current => {
                const next = current === 'light' ? 'dark' : 'light';
                if (browser) {
                    localStorage.setItem('theme', next);
                    document.documentElement.classList.toggle('dark', next === 'dark');
                }
                return next;
            });
        },
        set: (value: 'light' | 'dark') => {
            set(value);
            if (browser) {
                localStorage.setItem('theme', value);
                document.documentElement.classList.toggle('dark', value === 'dark');
            }
        }
    };
}

export const theme = createThemeStore();
```

- [ ] **Step 2: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/lib/stores/theme.ts
git commit -m "feat: add theme store with localStorage persistence"
```

---

### Task 12: Create ThemeToggle Component

**Files:**
- Create: `screen-time-app/src/lib/components/ThemeToggle.svelte`

**Interfaces:**
- Consumes: `theme` store from `$lib/stores/theme`

- [ ] **Step 1: Create `ThemeToggle.svelte`**

```svelte
<script lang="ts">
    import { theme } from '$lib/stores/theme';
</script>

<button
    class="text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low p-sm rounded-full transition-colors flex items-center justify-center"
    onclick={() => theme.toggle()}
    aria-label="Toggle dark mode"
>
    {#if $theme === 'dark'}
        <span class="material-symbols-outlined">light_mode</span>
    {:else}
        <span class="material-symbols-outlined">dark_mode</span>
    {/if}
</button>
```

- [ ] **Step 2: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/lib/components/ThemeToggle.svelte
git commit -m "feat: add ThemeToggle component"
```

---

### Task 13: Wire ThemeToggle into TopBar

**Files:**
- Modify: `screen-time-app/src/lib/components/TopBar.svelte`

- [ ] **Step 1: Verify ThemeToggle import exists from Task 1**

Check that `TopBar.svelte` already has `import ThemeToggle from './ThemeToggle.svelte';` and `<ThemeToggle />` in the header actions from Task 1, Step 1-2. If not, add them now.

- [ ] **Step 2: Verify with `npm run check && npm run dev`**

Run: `npm run check && npm run dev` (in `screen-time-app/`)
Expected: Sun/moon icon appears in top bar, clicking toggles dark/light

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/lib/components/TopBar.svelte
git commit -m "feat: wire ThemeToggle into TopBar"
```

---

### Task 14: Add Dark Mode Variants to All Components

**Files:**
- Modify: `screen-time-app/src/lib/components/Sidebar.svelte`
- Modify: `screen-time-app/src/lib/components/AppBlockCard.svelte`

**Pattern:** For each component, ensure every surface/color class has a `dark:` variant using M3 tokens:
- `bg-surface-container-lowest` → `dark:bg-surface-container`
- `border-outline-variant/30` → `dark:border-outline/20`
- `text-on-surface` stays same (M3 dark tokens already defined)
- `bg-surface-container-high` → `dark:bg-surface-container-highest`

- [ ] **Step 1: Audit Sidebar.svelte**

Sidebar already has `dark:` variants (verified from file read). No changes needed.

- [ ] **Step 2: Add `dark:` variants to AppBlockCard.svelte**

The toggle uses hardcoded colors. Update the un-toggled state: `bg-surface-variant` → add `dark:bg-surface-container-highest`.

- [ ] **Step 3: Verify dark mode in browser**

Run: `npm run dev` (in `screen-time-app/`)
Expected: Toggle theme, all pages render correctly in dark mode with proper contrast

- [ ] **Step 4: Commit**

```bash
git add screen-time-app/src/lib/components/
git commit -m "feat: add dark mode variants to all components"
```

---

## Phase 3: Backend Connection

### Task 15: Add Rust Backend Commands and Structs

**Files:**
- Modify: `screen-time-app/src-tauri/src/lib.rs`

**Interfaces:**
- Produces: `DailySummary`, `ProductivityDay`, `DeepWorkSession` structs
- Produces: `get_daily_summary`, `get_productivity_by_week`, `get_deep_work_sessions` commands

- [ ] **Step 1: Add new structs after the existing `Activity` struct**

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct DailySummary {
    pub total_duration: i64,
    pub productivity_score: i64,
    pub app_usage: Vec<AppUsage>,
    pub categories: Vec<CategoryBreakdown>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppUsage {
    pub app_name: String,
    pub duration: i64,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryBreakdown {
    pub name: String,
    pub duration: i64,
    pub percentage: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductivityDay {
    pub day: String,
    pub productive_duration: i64,
    pub neutral_duration: i64,
    pub leisure_duration: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeepWorkSession {
    pub app_name: String,
    pub title: String,
    pub start_time: String,
    pub end_time: String,
    pub duration: i64,
    pub category: String,
}
```

- [ ] **Step 2: Add `get_daily_summary` command**

```rust
#[tauri::command]
fn get_daily_summary(state: State<'_, AppState>) -> Result<DailySummary, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let start = format!("{}T00:00:00", today);
        let end = format!("{}T23:59:59", today);

        let mut stmt = conn.prepare(
            "SELECT app_name, duration, category, productivity_score FROM activities WHERE start_time >= ?1 AND start_time <= ?2"
        ).map_err(|e| e.to_string())?;

        let rows: Vec<(String, i64, String, i64)> = stmt.query_map(params![start, end], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        }).map_err(|e| e.to_string())?
          .filter_map(|r| r.ok()).collect();

        let total_duration: i64 = rows.iter().map(|r| r.1).sum();
        let productive_duration: i64 = rows.iter().filter(|r| r.3 > 0).map(|r| r.1).sum();
        let productivity_score = if total_duration > 0 {
            productive_duration * 100 / total_duration
        } else { 0 };

        let mut app_map: std::collections::HashMap<String, (i64, String)> = std::collections::HashMap::new();
        for (app, dur, cat, _) in &rows {
            let entry = app_map.entry(app.clone()).or_insert((0, cat.clone()));
            entry.0 += dur;
        }
        let mut app_usage: Vec<AppUsage> = app_map.iter()
            .map(|(k, (d, c))| AppUsage { app_name: k.clone(), duration: *d, category: c.clone() })
            .collect();
        app_usage.sort_by(|a, b| b.duration.cmp(&a.duration));

        let mut cat_map: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        for (_, dur, cat, _) in &rows {
            *cat_map.entry(cat.clone()).or_insert(0) += dur;
        }
        let mut categories: Vec<CategoryBreakdown> = cat_map.iter()
            .map(|(k, d)| CategoryBreakdown {
                name: k.clone(),
                duration: *d,
                percentage: if total_duration > 0 { d * 100 / total_duration } else { 0 },
            })
            .collect();
        categories.sort_by(|a, b| b.duration.cmp(&a.duration));

        Ok(DailySummary { total_duration, productivity_score, app_usage, categories })
    } else {
        Err("Database connection not initialized".to_string())
    }
}
```

- [ ] **Step 3: Add `get_productivity_by_week` command**

```rust
#[tauri::command]
fn get_productivity_by_week(state: State<'_, AppState>) -> Result<Vec<ProductivityDay>, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let today = Utc::now();
        let week_start = today - Duration::days(6);
        let start = week_start.format("%Y-%m-%dT00:00:00").to_string();

        let mut stmt = conn.prepare(
            "SELECT start_time, duration, productivity_score FROM activities WHERE start_time >= ?1"
        ).map_err(|e| e.to_string())?;

        let rows: Vec<(String, i64, i64)> = stmt.query_map(params![start], |row| {
            Ok((row.get::<_, String>(0)?, row.get(1)?, row.get(2)?))
        }).map_err(|e| e.to_string())?
          .filter_map(|r| r.ok()).collect();

        let day_names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        let days: Vec<ProductivityDay> = (0..7).map(|i| {
            let d = week_start + Duration::days(i);
            let day_str = d.format("%Y-%m-%d").to_string();
            let day_idx = d.format("%u").to_string().parse::<usize>().unwrap() - 1;
            let day_label = day_names[day_idx];

            let day_rows: Vec<_> = rows.iter().filter(|r| r.0.starts_with(&day_str)).collect();
            let productive: i64 = day_rows.iter().filter(|r| r.2 > 0).map(|r| r.1).sum();
            let total: i64 = day_rows.iter().map(|r| r.1).sum();
            let leisure: i64 = day_rows.iter().filter(|r| r.2 < 0).map(|r| r.1).sum();
            let neutral = total - productive - leisure;

            ProductivityDay {
                day: day_label.to_string(),
                productive_duration: productive,
                neutral_duration: neutral,
                leisure_duration: leisure,
            }
        }).collect();

        Ok(days)
    } else {
        Err("Database connection not initialized".to_string())
    }
}
```

- [ ] **Step 4: Add `get_deep_work_sessions` command**

```rust
#[tauri::command]
fn get_deep_work_sessions(state: State<'_, AppState>) -> Result<Vec<DeepWorkSession>, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let mut stmt = conn.prepare(
            "SELECT app_name, title, start_time, end_time, duration, category FROM activities WHERE productivity_score > 0 AND duration >= 1800 ORDER BY start_time DESC LIMIT 10"
        ).map_err(|e| e.to_string())?;

        let sessions: Vec<DeepWorkSession> = stmt.query_map([], |row| {
            Ok(DeepWorkSession {
                app_name: row.get(0)?,
                title: row.get(1)?,
                start_time: row.get(2)?,
                end_time: row.get(3)?,
                duration: row.get(4)?,
                category: row.get(5)?,
            })
        }).map_err(|e| e.to_string())?
          .filter_map(|r| r.ok()).collect();

        Ok(sessions)
    } else {
        Err("Database connection not initialized".to_string())
    }
}
```

- [ ] **Step 5: Register new commands in `run()`**

Update `invoke_handler`:
```rust
.invoke_handler(tauri::generate_handler![
    get_activities,
    get_daily_summary,
    get_productivity_by_week,
    get_deep_work_sessions
])
```

- [ ] **Step 6: Verify Rust compiles**

Run: `cargo check` (in `screen-time-app/src-tauri/`)
Expected: Compiles without errors

- [ ] **Step 7: Commit**

```bash
git add screen-time-app/src-tauri/src/lib.rs
git commit -m "feat: add daily_summary, productivity_by_week, deep_work_sessions commands"
```

---

### Task 16: Create TimeRange Store and Update Activities Store

**Files:**
- Create: `screen-time-app/src/lib/stores/timeRange.ts`
- Modify: `screen-time-app/src/lib/stores/activities.ts`

**Interfaces:**
- `timeRange.ts` produces: `selectedRange` writable (`"Day" | "Week" | "Month"`)
- `activities.ts` gains: `dailySummary`, `productivityByWeek`, `deepWorkSessions` stores and fetch functions

- [ ] **Step 1: Create `timeRange.ts`**

```typescript
import { writable } from 'svelte/store';

export type TimeRange = 'Day' | 'Week' | 'Month';
export const selectedRange = writable<TimeRange>('Day');
```

- [ ] **Step 2: Add new stores and fetch functions to `activities.ts`**

Append to the existing file:

```typescript
export const dailySummary = writable<DailySummary | null>(null);
export const productivityByWeek = writable<ProductivityDay[]>([]);
export const deepWorkSessionsList = writable<DeepWorkSession[]>([]);

export interface DailySummary {
    total_duration: number;
    productivity_score: number;
    app_usage: BackendAppUsage[];
    categories: CategoryBreakdown[];
}

export interface BackendAppUsage {
    app_name: string;
    duration: number;
    category: string;
}

export interface CategoryBreakdown {
    name: string;
    duration: number;
    percentage: number;
}

export interface ProductivityDay {
    day: string;
    productive_duration: number;
    neutral_duration: number;
    leisure_duration: number;
}

export interface DeepWorkSession {
    app_name: string;
    title: string;
    start_time: string;
    end_time: string;
    duration: number;
    category: string;
}

export async function fetchDailySummary() {
    try {
        const data: DailySummary = await invoke('get_daily_summary');
        dailySummary.set(data);
    } catch (e) {
        console.error("Failed to fetch daily summary:", e);
    }
}

export async function fetchProductivityByWeek() {
    try {
        const data: ProductivityDay[] = await invoke('get_productivity_by_week');
        productivityByWeek.set(data);
    } catch (e) {
        console.error("Failed to fetch productivity data:", e);
    }
}

export async function fetchDeepWorkSessions() {
    try {
        const data: DeepWorkSession[] = await invoke('get_deep_work_sessions');
        deepWorkSessionsList.set(data);
    } catch (e) {
        console.error("Failed to fetch deep work sessions:", e);
    }
}
```

- [ ] **Step 3: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 4: Commit**

```bash
git add screen-time-app/src/lib/stores/timeRange.ts screen-time-app/src/lib/stores/activities.ts
git commit -m "feat: add timeRange store and backend-connected data stores"
```

---

### Task 17: Wire Overview Page to Backend Data

**Files:**
- Modify: `screen-time-app/src/routes/+page.svelte`

**Interfaces:**
- Consumes: `dailySummary`, `totalDuration`, `productivityScore` stores from `activities.ts`
- Uses: `formatDuration()` for display

- [ ] **Step 1: Update Overview page to use stores**

Replace the hardcoded data in the script block and template with store subscriptions:

```svelte
<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
    import StatCard from '$lib/components/StatCard.svelte';
    import AppUsageList from '$lib/components/AppUsageList.svelte';
    import CategoryDonut from '$lib/components/CategoryDonut.svelte';
    import TimeRangeSelector from '$lib/components/TimeRangeSelector.svelte';
    import { dailySummary, totalDuration, productivityScore, formatDuration } from '$lib/stores/activities';

    let summary = $derived($dailySummary);

    const categoryColors: Record<string, string> = {
        Coding: "#0058bc",
        Design: "#4c4aca",
        Communication: "#006e28",
        Entertainment: "#E50914",
        Neutral: "#e3e2e7",
    };
</script>

<TopBar title="Overview" subtitle="Today" />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-7xl mx-auto w-full">
    <div class="grid grid-cols-12 gap-lg">
        <div class="col-span-12 lg:col-span-4">
            <StatCard
                icon="schedule"
                label="Total Screen Time"
                value={formatDuration($totalDuration)}
                progress={$productivityScore}
            />
        </div>

        <div class="col-span-12 lg:col-span-8 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20 flex flex-col">
            <div class="flex justify-between items-center mb-xl">
                <h2 class="font-headline-md text-headline-md text-on-surface">Daily Usage</h2>
                <TimeRangeSelector />
            </div>
            <div class="flex-1 flex items-center justify-center min-h-[200px] text-on-surface-variant font-body-md">
                {#if summary && summary.app_usage.length > 0}
                    <p>Chart coming in Phase 4</p>
                {:else}
                    <p>No data yet today</p>
                {/if}
            </div>
        </div>

        <div class="col-span-12 lg:col-span-6 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20">
            {#if summary}
                <AppUsageList
                    items={summary.app_usage.slice(0, 5).map(a => ({
                        name: a.app_name,
                        duration: a.duration,
                        color: categoryColors[a.category] || "#e3e2e7",
                        icon: "apps"
                    }))}
                    totalDuration={summary.total_duration}
                />
            {:else}
                <p class="text-on-surface-variant font-body-md">Loading...</p>
            {/if}
        </div>

        <div class="col-span-12 lg:col-span-6 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20">
            {#if summary}
                <CategoryDonut
                    categories={summary.categories.map(c => ({
                        name: c.name,
                        percentage: c.percentage,
                        color: categoryColors[c.name] || "#e3e2e7"
                    }))}
                />
            {:else}
                <p class="text-on-surface-variant font-body-md">Loading...</p>
            {/if}
        </div>
    </div>
</main>
```

- [ ] **Step 2: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/routes/+page.svelte
git commit -m "feat: wire Overview page to backend daily summary"
```

---

### Task 18: Wire Productivity Page to Backend Data

**Files:**
- Modify: `screen-time-app/src/routes/productivity/+page.svelte`

**Interfaces:**
- Consumes: `productivityByWeek`, `deepWorkSessionsList` stores

- [ ] **Step 1: Update Productivity page to use stores**

```svelte
<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
    import StatCard from '$lib/components/StatCard.svelte';
    import ProductivityChart from '$lib/components/ProductivityChart.svelte';
    import DeepWorkTimeline from '$lib/components/DeepWorkTimeline.svelte';
    import { productivityScore, deepWorkSessionsList, totalDuration, formatDuration, productivityByWeek } from '$lib/stores/activities';

    let weekData = $derived($productivityByWeek.map(d => ({
        day: d.day,
        productive: d.productive_duration,
        neutral: d.neutral_duration,
        leisure: d.leisure_duration,
    })));

    let sessions = $derived($deepWorkSessionsList.map(s => ({
        title: s.title || s.app_name,
        startTime: new Date(s.start_time).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        endTime: new Date(s.end_time).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        duration: formatDuration(s.duration),
        category: s.category,
        color: s.category === 'Coding' ? '#0058bc' : s.category === 'Design' ? '#4c4aca' : '#006e28',
    })));
</script>

<TopBar title="Productivity Tracker" />

<main class="flex-1 overflow-y-auto p-margin-desktop mt-[88px]">
    <div class="max-w-[1200px] mx-auto grid grid-cols-12 gap-lg pb-xxl">
        <div class="col-span-12 md:col-span-4">
            <StatCard
                icon="trending_up"
                label="Productivity Score"
                value="{$productivityScore}%"
                progress={$productivityScore}
            />
        </div>

        <div class="col-span-12 md:col-span-8 bg-surface-container-lowest border border-outline-variant rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20">
            {#if weekData.length > 0}
                <ProductivityChart data={weekData} />
            {:else}
                <p class="text-on-surface-variant font-body-md p-lg">No productivity data yet</p>
            {/if}
        </div>

        <div class="col-span-12 md:col-span-7 bg-surface-container-lowest border border-outline-variant rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20">
            {#if sessions.length > 0}
                <DeepWorkTimeline sessions={sessions} />
            {:else}
                <p class="text-on-surface-variant font-body-md p-lg">No deep work sessions recorded yet</p>
            {/if}
        </div>
    </div>
</main>
```

- [ ] **Step 2: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/routes/productivity/+page.svelte
git commit -m "feat: wire Productivity page to backend data"
```

---

### Task 19: Update Layout to Poll All Data

**Files:**
- Modify: `screen-time-app/src/routes/+layout.svelte`

**Interfaces:**
- Calls: `fetchDailySummary`, `fetchProductivityByWeek`, `fetchDeepWorkSessions` alongside existing `fetchActivities`

- [ ] **Step 1: Add new fetch calls to layout**

```svelte
<script lang="ts">
    import Sidebar from '$lib/components/Sidebar.svelte';
    import { onMount, onDestroy } from 'svelte';
    import { setupIdleListener } from '$lib/stores/idle';
    import { fetchActivities, fetchDailySummary, fetchProductivityByWeek, fetchDeepWorkSessions } from '$lib/stores/activities';
    import '../app.css';

    let { children, data } = $props();
    let unlisten: (() => void) | undefined;
    let pollInterval: ReturnType<typeof setInterval>;

    async function fetchAll() {
        await Promise.all([
            fetchActivities(),
            fetchDailySummary(),
            fetchProductivityByWeek(),
            fetchDeepWorkSessions(),
        ]);
    }

    onMount(async () => {
        try {
            unlisten = await setupIdleListener();
            await fetchAll();
            pollInterval = setInterval(fetchAll, 5000);
        } catch (e) {
            console.error("Failed to setup layout:", e);
        }
    });

    onDestroy(() => {
        if (unlisten) unlisten();
        if (pollInterval) clearInterval(pollInterval);
    });
</script>

<div class="flex min-h-screen">
    <Sidebar />
    <div class="flex-1 ml-[280px] flex flex-col min-h-screen">
        {@render children()}
    </div>
</div>
```

- [ ] **Step 2: Verify with `npm run check && npm run build`**

Run: `npm run check && npm run build` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/routes/+layout.svelte
git commit -m "feat: poll all backend data stores in layout"
```

---

## Phase 4: Real Charting (Chart.js)

### Task 20: Install Chart.js Dependencies

**Files:**
- Modify: `screen-time-app/package.json`

- [ ] **Step 1: Install chart.js and svelte-chartjs**

Run (in `screen-time-app/`):
```bash
npm install chart.js svelte-chartjs
```

- [ ] **Step 2: Verify installation**

Run: `npm ls chart.js svelte-chartjs`
Expected: Both listed as installed

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/package.json screen-time-app/package-lock.json
git commit -m "chore: install chart.js and svelte-chartjs"
```

---

### Task 21: Create Real BarChart Component

**Files:**
- Create: `screen-time-app/src/lib/components/BarChart.svelte`
- Modify: `screen-time-app/src/routes/+page.svelte`

**Interfaces:**
- Consumes: `labels` (string[]), `data` (number[]), `unit` (string, optional)

- [ ] **Step 1: Create `BarChart.svelte`**

```svelte
<script lang="ts">
    import { onMount } from 'svelte';
    import { Chart, registerables } from 'chart.js';

    Chart.register(...registerables);

    let { labels, data, unit = "hours" }: {
        labels: string[];
        data: number[];
        unit?: string;
    } = $props();

    let canvas: HTMLCanvasElement;

    onMount(() => {
        const chart = new Chart(canvas, {
            type: 'bar',
            data: {
                labels,
                datasets: [{
                    data,
                    backgroundColor: 'rgba(0, 88, 188, 0.6)',
                    borderColor: 'rgba(0, 88, 188, 1)',
                    borderWidth: 1,
                    borderRadius: 8,
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: { display: false },
                    tooltip: {
                        callbacks: {
                            label: (ctx) => `${ctx.parsed.y.toFixed(1)} ${unit}`
                        }
                    }
                },
                scales: {
                    y: {
                        beginAtZero: true,
                        grid: { color: 'rgba(0,0,0,0.05)' },
                        ticks: { font: { family: 'Inter', size: 11 } }
                    },
                    x: {
                        grid: { display: false },
                        ticks: { font: { family: 'Inter', size: 11 } }
                    }
                }
            }
        });

        return () => chart.destroy();
    });
</script>

<div class="relative w-full h-full min-h-[200px]">
    <canvas bind:this={canvas}></canvas>
</div>
```

- [ ] **Step 2: Update Overview page to use BarChart**

In `+page.svelte`, replace the placeholder chart section with:
```svelte
<div class="col-span-12 lg:col-span-8 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20 flex flex-col">
    <div class="flex justify-between items-center mb-xl">
        <h2 class="font-headline-md text-headline-md text-on-surface">Daily Usage</h2>
        <TimeRangeSelector />
    </div>
    <div class="flex-1 min-h-[200px]">
        {#if summary && summary.app_usage.length > 0}
            <BarChart
                labels={summary.app_usage.slice(0, 6).map(a => a.app_name)}
                data={summary.app_usage.slice(0, 6).map(a => a.duration / 3600)}
                unit="hours"
            />
        {:else}
            <div class="flex items-center justify-center h-full text-on-surface-variant font-body-md">No data yet today</div>
        {/if}
    </div>
</div>
```

Add `import BarChart from '$lib/components/BarChart.svelte';` to the script block.

- [ ] **Step 3: Verify with `npm run check && npm run build`**

Run: `npm run check && npm run build` (in `screen-time-app/`)
Expected: Pass, chart renders with real data

- [ ] **Step 4: Commit**

```bash
git add screen-time-app/src/lib/components/BarChart.svelte screen-time-app/src/routes/+page.svelte
git commit -m "feat: add real Chart.js BarChart to Overview page"
```

---

### Task 22: Create Real DonutChart Component

**Files:**
- Create: `screen-time-app/src/lib/components/DonutChart.svelte`
- Modify: `screen-time-app/src/lib/components/CategoryDonut.svelte`

**Interfaces:**
- Consumes: `labels` (string[]), `data` (number[]), `colors` (string[])

- [ ] **Step 1: Create `DonutChart.svelte`**

```svelte
<script lang="ts">
    import { onMount } from 'svelte';
    import { Chart, registerables } from 'chart.js';

    Chart.register(...registerables);

    let { labels, data, colors }: {
        labels: string[];
        data: number[];
        colors: string[];
    } = $props();

    let canvas: HTMLCanvasElement;

    onMount(() => {
        const chart = new Chart(canvas, {
            type: 'doughnut',
            data: {
                labels,
                datasets: [{
                    data,
                    backgroundColor: colors,
                    borderWidth: 0,
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: true,
                cutout: '65%',
                plugins: {
                    legend: { display: false },
                }
            }
        });

        return () => chart.destroy();
    });
</script>

<div class="relative w-32 h-32 flex-shrink-0">
    <canvas bind:this={canvas}></canvas>
</div>
```

- [ ] **Step 2: Update CategoryDonut to use DonutChart**

Update `CategoryDonut.svelte` to import and use `DonutChart` instead of the SVG:

```svelte
<script lang="ts">
    import DonutChart from './DonutChart.svelte';

    let { categories }: {
        categories: Array<{ name: string; percentage: number; color: string }>;
    } = $props();
</script>

<h2 class="font-headline-md text-headline-md text-on-surface mb-xl">Categories</h2>
<div class="flex gap-lg items-center">
    <DonutChart
        labels={categories.map(c => c.name)}
        data={categories.map(c => c.percentage)}
        colors={categories.map(c => c.color)}
    />
    <div class="flex-1 flex flex-col gap-md">
        {#each categories as cat}
            <div class="flex justify-between items-center">
                <div class="flex items-center gap-sm">
                    <div class="w-3 h-3 rounded-full" style="background-color: {cat.color}"></div>
                    <span class="font-body-md text-body-md text-on-surface">{cat.name}</span>
                </div>
                <span class="font-label-md text-label-md text-on-surface-variant">{cat.percentage}%</span>
            </div>
        {/each}
    </div>
</div>
```

- [ ] **Step 3: Verify with `npm run check && npm run build`**

Run: `npm run check && npm run build` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 4: Commit**

```bash
git add screen-time-app/src/lib/components/DonutChart.svelte screen-time-app/src/lib/components/CategoryDonut.svelte
git commit -m "feat: add Chart.js DonutChart to CategoryDonut"
```

---

### Task 23: Create Real Productivity Line Chart

**Files:**
- Modify: `screen-time-app/src/lib/components/ProductivityChart.svelte`

**Interfaces:**
- Consumes: `data` array of `{ day, productive, neutral, leisure }` (same interface, now uses Chart.js)

- [ ] **Step 1: Rewrite ProductivityChart with Chart.js stacked bar**

```svelte
<script lang="ts">
    import { onMount } from 'svelte';
    import { Chart, registerables } from 'chart.js';

    Chart.register(...registerables);

    let { data }: {
        data: Array<{ day: string; productive: number; neutral: number; leisure: number }>;
    } = $props();

    let canvas: HTMLCanvasElement;

    onMount(() => {
        const chart = new Chart(canvas, {
            type: 'bar',
            data: {
                labels: data.map(d => d.day),
                datasets: [
                    {
                        label: 'Productive',
                        data: data.map(d => d.productive / 3600),
                        backgroundColor: 'rgba(0, 88, 188, 0.8)',
                        borderRadius: { topLeft: 0, topRight: 0, bottomLeft: 4, bottomRight: 4 },
                    },
                    {
                        label: 'Neutral',
                        data: data.map(d => d.neutral / 3600),
                        backgroundColor: 'rgba(227, 226, 231, 0.8)',
                    },
                    {
                        label: 'Leisure',
                        data: data.map(d => d.leisure / 3600),
                        backgroundColor: 'rgba(76, 74, 202, 0.8)',
                        borderRadius: { topLeft: 4, topRight: 4, bottomLeft: 0, bottomRight: 0 },
                    },
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        position: 'top',
                        labels: { font: { family: 'Inter', size: 11 }, usePointStyle: true, pointStyle: 'circle' }
                    },
                },
                scales: {
                    x: { stacked: true, grid: { display: false }, ticks: { font: { family: 'Inter', size: 11 } } },
                    y: { stacked: true, beginAtZero: true, grid: { color: 'rgba(0,0,0,0.05)' }, ticks: { font: { family: 'Inter', size: 11 } } },
                }
            }
        });

        return () => chart.destroy();
    });
</script>

<h3 class="font-headline-md text-headline-md text-on-surface font-semibold mb-lg">Work vs. Leisure</h3>
<div class="relative h-48 mt-md">
    <canvas bind:this={canvas}></canvas>
</div>
```

- [ ] **Step 2: Verify with `npm run check && npm run build`**

Run: `npm run check && npm run build` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/lib/components/ProductivityChart.svelte
git commit -m "feat: replace ProductivityChart with Chart.js stacked bar"
```

---

## Phase 5: Blocked Apps System

### Task 24: Add Blocked Apps Database Table and Backend Commands

**Files:**
- Modify: `screen-time-app/src-tauri/src/lib.rs`

**Interfaces:**
- Produces: `BlockedApp` struct, `get_blocked_apps`, `add_blocked_app`, `remove_blocked_app`, `toggle_blocked_app` commands
- Produces: `pub fn is_app_blocked()` helper for tracker integration

- [ ] **Step 1: Add `blocked_apps` table to `init_db`**

After the existing `settings` table creation in `init_db`:

```rust
conn.execute("
    CREATE TABLE IF NOT EXISTS blocked_apps (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        app_name TEXT NOT NULL UNIQUE,
        is_blocked INTEGER NOT NULL DEFAULT 1,
        created_at TEXT NOT NULL DEFAULT (datetime('now'))
    );
", [])?;
```

- [ ] **Step 2: Add `BlockedApp` struct**

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockedApp {
    pub id: i64,
    pub app_name: String,
    pub is_blocked: bool,
}
```

- [ ] **Step 3: Add blocked apps CRUD commands**

```rust
#[tauri::command]
fn get_blocked_apps(state: State<'_, AppState>) -> Result<Vec<BlockedApp>, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        let mut stmt = conn.prepare("SELECT id, app_name, is_blocked FROM blocked_apps")
            .map_err(|e| e.to_string())?;
        let apps = stmt.query_map([], |row| {
            Ok(BlockedApp {
                id: row.get(0)?,
                app_name: row.get(1)?,
                is_blocked: row.get::<_, i64>(2)? == 1,
            })
        }).map_err(|e| e.to_string())?
          .filter_map(|r| r.ok()).collect();
        Ok(apps)
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn add_blocked_app(state: State<'_, AppState>, app_name: String) -> Result<BlockedApp, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        conn.execute("INSERT INTO blocked_apps (app_name, is_blocked) VALUES (?1, 1)", params![app_name])
            .map_err(|e| e.to_string())?;
        let id = conn.last_insert_rowid();
        Ok(BlockedApp { id, app_name, is_blocked: true })
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn remove_blocked_app(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        conn.execute("DELETE FROM blocked_apps WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
fn toggle_blocked_app(state: State<'_, AppState>, id: i64) -> Result<BlockedApp, String> {
    let conn_guard = state.conn.lock().unwrap();
    if let Some(conn) = conn_guard.as_ref() {
        conn.execute("UPDATE blocked_apps SET is_blocked = NOT is_blocked WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        let app = conn.query_row("SELECT id, app_name, is_blocked FROM blocked_apps WHERE id = ?1", params![id], |row| {
            Ok(BlockedApp {
                id: row.get(0)?,
                app_name: row.get(1)?,
                is_blocked: row.get::<_, i64>(2)? == 1,
            })
        }).map_err(|e| e.to_string())?;
        Ok(app)
    } else {
        Err("Database not initialized".to_string())
    }
}
```

- [ ] **Step 4: Register new commands**

Update `invoke_handler`:
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
])
```

- [ ] **Step 5: Add `is_app_blocked` helper**

```rust
pub fn is_app_blocked(conn: &Connection, app_name: &str) -> bool {
    conn.query_row(
        "SELECT COUNT(*) FROM blocked_apps WHERE app_name = ?1 AND is_blocked = 1",
        params![app_name],
        |row| row.get::<_, i64>(0),
    ).unwrap_or(0) > 0
}
```

- [ ] **Step 6: Verify Rust compiles**

Run: `cargo check` (in `screen-time-app/src-tauri/`)
Expected: Pass

- [ ] **Step 7: Commit**

```bash
git add screen-time-app/src-tauri/src/lib.rs
git commit -m "feat: add blocked_apps table and CRUD commands"
```

---

### Task 25: Create Blocker Enforcement Module

**Files:**
- Create: `screen-time-app/src-tauri/src/blocker.rs`

**Interfaces:**
- Produces: `enforce_blocked_apps(conn, app_name)` function
- Uses: WM-specific commands (Hyprland `hyprctl`, Sway `swaymsg`, X11 `wmctrl`)

- [ ] **Step 1: Create `blocker.rs`**

```rust
use rusqlite::Connection;
use std::process::Command;
use crate::lib::is_app_blocked;

pub fn enforce_blocked_apps(conn: &Connection, app_name: &str) -> bool {
    if !is_app_blocked(conn, app_name) {
        return false;
    }

    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "x11".to_string());

    match session_type.as_str() {
        "wayland" => enforce_wayland(app_name),
        _ => enforce_x11(app_name),
    }

    true
}

fn enforce_wayland(app_name: &str) {
    // Try Hyprland
    if Command::new("hyprctl").arg("clients").output().is_ok() {
        let _ = Command::new("hyprctl")
            .args(&["dispatch", "killactive", &format!("class:{}", app_name)])
            .output();
        return;
    }

    // Try Sway
    if let Ok(output) = Command::new("swaymsg").arg("-t").arg("get_tree").output() {
        if output.status.success() {
            let close_cmd = format!("[app_id=\"{}\"] kill", app_name);
            let _ = Command::new("swaymsg").arg(&close_cmd).output();
        }
    }
}

fn enforce_x11(app_name: &str) {
    if let Ok(output) = Command::new("wmctrl").arg("-l").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.to_lowercase().contains(&app_name.to_lowercase()) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(window_id) = parts.first() {
                        let _ = Command::new("wmctrl").args(&["-i", "-c", window_id]).output();
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 2: Add `mod blocker;` to `lib.rs`**

Add to the module declarations:
```rust
mod blocker;
```

- [ ] **Step 3: Verify Rust compiles**

Run: `cargo check` (in `screen-time-app/src-tauri/`)
Expected: Pass

- [ ] **Step 4: Commit**

```bash
git add screen-time-app/src-tauri/src/blocker.rs screen-time-app/src-tauri/src/lib.rs
git commit -m "feat: add blocker enforcement module for Hyprland/Sway/X11"
```

---

### Task 26: Integrate Blocker into Tracker Polling Loop

**Files:**
- Modify: `screen-time-app/src-tauri/src/tracker.rs`

**Interfaces:**
- Uses: `blocker::enforce_blocked_apps()`, receives `conn` (already available)

- [ ] **Step 1: Add blocker check after detecting active window**

In `tracker.rs`, inside the main loop, after `if let Some(active) = get_active_window()` and before the category/score logic, add:

```rust
// Check if this app is blocked
if let Ok(db_guard) = conn.lock() {
    if let Some(db) = db_guard.as_ref() {
        if crate::blocker::enforce_blocked_apps(db, &active.app_name) {
            current_window = None;
            current_id = None;
            continue;
        }
    }
}
```

- [ ] **Step 2: Verify Rust compiles**

Run: `cargo check` (in `screen-time-app/src-tauri/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src-tauri/src/tracker.rs
git commit -m "feat: integrate blocker check into tracker polling loop"
```

---

### Task 27: Create Blocked Apps Frontend Store

**Files:**
- Create: `screen-time-app/src/lib/stores/blockedApps.ts`

**Interfaces:**
- Produces: `blockedApps` store, `fetchBlockedApps`, `addBlockedApp`, `removeBlockedApp`, `toggleBlockedApp`

- [ ] **Step 1: Create `blockedApps.ts`**

```typescript
import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface BlockedApp {
    id: number;
    app_name: string;
    is_blocked: boolean;
}

export const blockedApps = writable<BlockedApp[]>([]);

export async function fetchBlockedApps() {
    try {
        const data: BlockedApp[] = await invoke('get_blocked_apps');
        blockedApps.set(data);
    } catch (e) {
        console.error("Failed to fetch blocked apps:", e);
    }
}

export async function addBlockedApp(appName: string) {
    try {
        await invoke('add_blocked_app', { appName });
        await fetchBlockedApps();
    } catch (e) {
        console.error("Failed to add blocked app:", e);
    }
}

export async function removeBlockedApp(id: number) {
    try {
        await invoke('remove_blocked_app', { id });
        await fetchBlockedApps();
    } catch (e) {
        console.error("Failed to remove blocked app:", e);
    }
}

export async function toggleBlockedApp(id: number) {
    try {
        await invoke('toggle_blocked_app', { id });
        await fetchBlockedApps();
    } catch (e) {
        console.error("Failed to toggle blocked app:", e);
    }
}
```

- [ ] **Step 2: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/lib/stores/blockedApps.ts
git commit -m "feat: add blockedApps frontend store"
```

---

### Task 28: Rewrite Blocked Page with Real Data

**Files:**
- Modify: `screen-time-app/src/routes/blocked/+page.svelte`

**Interfaces:**
- Consumes: `blockedApps`, `toggleBlockedApp`, `fetchBlockedApps` from `blockedApps.ts`
- Consumes: `AppBlockCard`, `PageHeader`

- [ ] **Step 1: Rewrite Blocked page with backend data**

```svelte
<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
    import PageHeader from '$lib/components/PageHeader.svelte';
    import AppBlockCard from '$lib/components/AppBlockCard.svelte';
    import { blockedApps, toggleBlockedApp, fetchBlockedApps } from '$lib/stores/blockedApps';
    import { onMount } from 'svelte';

    onMount(() => {
        fetchBlockedApps();
    });

    function categoryForApp(appName: string): string {
        const social = ['instagram', 'tiktok', 'twitter', 'facebook', 'reddit', 'discord', 'slack'];
        const entertainment = ['youtube', 'netflix', 'spotify', 'twitch', 'steam'];
        const lower = appName.toLowerCase();
        if (social.some(s => lower.includes(s))) return 'Social Media';
        if (entertainment.some(e => lower.includes(e))) return 'Entertainment';
        return 'Other';
    }

    let socialApps = $derived(
        $blockedApps.filter(a => categoryForApp(a.app_name) === 'Social Media')
    );
    let entertainmentApps = $derived(
        $blockedApps.filter(a => categoryForApp(a.app_name) === 'Entertainment')
    );
    let otherApps = $derived(
        $blockedApps.filter(a => categoryForApp(a.app_name) === 'Other')
    );
</script>

<TopBar title="Screen Time" />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-7xl mx-auto w-full">
    <div class="max-w-4xl mx-auto space-y-xl">
        <PageHeader title="Blocked Apps & Limits" description="Manage restrictions and focus schedules for your applications." />

        {#if $blockedApps.length === 0}
            <div class="text-center py-xxl">
                <p class="text-on-surface-variant font-body-md">No blocked apps configured. Add apps to start blocking.</p>
            </div>
        {/if}

        {#if socialApps.length > 0}
            <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
                <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                    <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Social Media</h3>
                    <span class="font-label-sm text-label-sm text-on-surface-variant bg-surface-variant/50 px-sm py-xs rounded-md">{socialApps.length} Apps</span>
                </div>
                <div class="divide-y divide-outline-variant/20">
                    {#each socialApps as app}
                        <AppBlockCard
                            appName={app.app_name}
                            icon="smart_display"
                            iconBg="rgba(0,0,0,0.05)"
                            limit={app.is_blocked ? "Blocked" : "Allowed"}
                            isBlocked={app.is_blocked}
                            onToggle={() => toggleBlockedApp(app.id)}
                        />
                    {/each}
                </div>
            </section>
        {/if}

        {#if entertainmentApps.length > 0}
            <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
                <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                    <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Entertainment</h3>
                    <span class="font-label-sm text-label-sm text-on-surface-variant bg-surface-variant/50 px-sm py-xs rounded-md">{entertainmentApps.length} Apps</span>
                </div>
                <div class="divide-y divide-outline-variant/20">
                    {#each entertainmentApps as app}
                        <AppBlockCard
                            appName={app.app_name}
                            icon="play_arrow"
                            iconBg="rgba(255,0,0,0.1)"
                            limit={app.is_blocked ? "Blocked" : "Allowed"}
                            isBlocked={app.is_blocked}
                            onToggle={() => toggleBlockedApp(app.id)}
                        />
                    {/each}
                </div>
            </section>
        {/if}

        {#if otherApps.length > 0}
            <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
                <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                    <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Other</h3>
                    <span class="font-label-sm text-label-sm text-on-surface-variant bg-surface-variant/50 px-sm py-xs rounded-md">{otherApps.length} Apps</span>
                </div>
                <div class="divide-y divide-outline-variant/20">
                    {#each otherApps as app}
                        <AppBlockCard
                            appName={app.app_name}
                            icon="apps"
                            iconBg="rgba(0,0,0,0.05)"
                            limit={app.is_blocked ? "Blocked" : "Allowed"}
                            isBlocked={app.is_blocked}
                            onToggle={() => toggleBlockedApp(app.id)}
                        />
                    {/each}
                </div>
            </section>
        {/if}
    </div>
</main>
```

- [ ] **Step 2: Verify with `npm run check && npm run build`**

Run: `npm run check && npm run build` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/routes/blocked/+page.svelte
git commit -m "feat: rewrite Blocked page with backend data and toggle"
```

---

### Task 29: Create AddAppModal Component

**Files:**
- Create: `screen-time-app/src/lib/components/AddAppModal.svelte`

**Interfaces:**
- Consumes: `addBlockedApp` from `blockedApps.ts`
- Props: `open` (boolean), `onclose` callback

- [ ] **Step 1: Create `AddAppModal.svelte`**

```svelte
<script lang="ts">
    import { addBlockedApp } from '$lib/stores/blockedApps';

    let { open = false, onclose }: { open: boolean; onclose: () => void } = $props();
    let appName = $state('');

    async function handleSubmit() {
        if (appName.trim()) {
            await addBlockedApp(appName.trim());
            appName = '';
            onclose();
        }
    }
</script>

{#if open}
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div class="fixed inset-0 z-[100] flex items-center justify-center bg-black/40" onclick={onclose}>
        <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
        <div class="bg-surface-container-lowest border border-outline-variant/30 rounded-2xl p-xl w-full max-w-md shadow-xl" onclick={(e) => e.stopPropagation()}>
            <h3 class="font-headline-md text-headline-md text-on-surface mb-lg">Block an App</h3>
            <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
                <label class="font-label-md text-label-md text-on-surface-variant block mb-sm">Application Name</label>
                <input
                    type="text"
                    bind:value={appName}
                    placeholder="e.g. Instagram, YouTube"
                    class="w-full px-md py-sm rounded-lg border border-outline-variant bg-surface text-on-surface font-body-md focus:outline-none focus:border-primary"
                />
                <div class="flex justify-end gap-sm mt-lg">
                    <button type="button" class="px-md py-sm rounded-lg font-label-md text-label-md text-on-surface-variant hover:bg-surface-container-low" onclick={onclose}>Cancel</button>
                    <button type="submit" class="px-md py-sm rounded-lg font-label-md text-label-md text-on-primary bg-primary hover:bg-primary-container">Block</button>
                </div>
            </form>
        </div>
    </div>
{/if}
```

- [ ] **Step 2: Verify with `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/lib/components/AddAppModal.svelte
git commit -m "feat: add AddAppModal component"
```

---

### Task 30: Integrate AddAppModal into Blocked Page

**Files:**
- Modify: `screen-time-app/src/routes/blocked/+page.svelte`

**Interfaces:**
- Consumes: `AddAppModal`

- [ ] **Step 1: Add modal state and button to Blocked page**

In the script block, add:
```typescript
import AddAppModal from '$lib/components/AddAppModal.svelte';
let showModal = $state(false);
```

After the `PageHeader`, add a floating add button:
```svelte
<div class="flex justify-end mb-lg">
    <button
        class="bg-primary text-on-primary px-md py-sm rounded-lg font-label-md text-label-md hover:bg-primary-container transition-colors flex items-center gap-xs"
        onclick={() => showModal = true}
    >
        <span class="material-symbols-outlined text-[18px]">add</span>
        Block App
    </button>
</div>
```

At the bottom of the template (before closing `</main>`), add:
```svelte
<AddAppModal open={showModal} onclose={() => showModal = false} />
```

- [ ] **Step 2: Verify with `npm run check && npm run build`**

Run: `npm run check && npm run build` (in `screen-time-app/`)
Expected: Pass

- [ ] **Step 3: Commit**

```bash
git add screen-time-app/src/routes/blocked/+page.svelte
git commit -m "feat: integrate AddAppModal into Blocked page"
```

---

## Phase 6: Final Polish

### Task 31: Global Dark Mode Audit and Toggle Fix

**Files:**
- Modify: `screen-time-app/src/app.css`

- [ ] **Step 1: Update toggle CSS to use theme-aware colors**

In `app.css`, replace the hardcoded toggle colors:

```css
.toggle-checkbox:checked {
    right: 0;
    border-color: #0058bc;
}
.toggle-checkbox:checked + .toggle-label {
    background-color: #0058bc;
}
```

These already use the primary hex, so they should work in both themes. Verify no toggle elements appear invisible in dark mode.

- [ ] **Step 2: Verify dark mode across all pages**

Run: `npm run dev` (in `screen-time-app/`)
Expected: Toggle dark mode, all pages (Overview, Productivity, Blocked) render correctly with proper contrast and no white-on-white or invisible elements

- [ ] **Step 3: Final build check**

Run: `npm run check && npm run build` (in `screen-time-app/`)
Expected: Clean build with no errors

- [ ] **Step 4: Commit**

```bash
git add screen-time-app/src/app.css
git commit -m "fix: verify toggle CSS works in dark mode"
```

---

### Task 32: Final Verification

- [ ] **Step 1: Full Rust build**

Run: `cargo build` (in `screen-time-app/src-tauri/`)
Expected: Builds successfully

- [ ] **Step 2: Full frontend build**

Run: `npm run build` (in `screen-time-app/`)
Expected: Clean build

- [ ] **Step 3: Run `npm run check`**

Run: `npm run check` (in `screen-time-app/`)
Expected: No TypeScript errors

- [ ] **Step 4: Manual smoke test checklist**

If possible, run `npm run tauri dev` and verify:
- [ ] Overview page shows real data from backend (or empty state if no data)
- [ ] Productivity page renders charts and deep work sessions
- [ ] Blocked page shows blocked apps from DB
- [ ] Add app modal works, adds app to DB
- [ ] Toggle blocks/unblocks app
- [ ] Dark mode toggle works on all pages
- [ ] Theme persists across page reloads (localStorage)

- [ ] **Step 5: Final commit**

```bash
git add -A
git commit -m "chore: final verification and polish"
```
