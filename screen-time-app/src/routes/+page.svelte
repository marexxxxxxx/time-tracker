<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';

    interface Activity {
        id: number;
        app_name: string;
        title: string;
        start_time: string;
        end_time: string;
        duration: number;
        category: string;
        productivity_score: number;
    }

    let activities: Activity[] = [];
    let totalDurationToday = 0;
    let topApps: { app_name: string; duration: number; percentage: number }[] = [];
    let hourlyUsage: number[] = Array(24).fill(0);

    // Simplification for the 6 bars (4 hours each)
    let binnedUsage: number[] = Array(6).fill(0);
    let maxBinUsage = 1; // avoid division by zero

    function formatDuration(seconds: number): string {
        const h = Math.floor(seconds / 3600);
        const m = Math.floor((seconds % 3600) / 60);
        if (h > 0 && m > 0) return `${h}h ${m}m`;
        if (h > 0) return `${h}h`;
        return `${m}m`;
    }

    let todayFormatted = "";

    onMount(async () => {
        try {
            const today = new Date();
            const options: Intl.DateTimeFormatOptions = { weekday: 'long', month: 'short', day: 'numeric' };
            todayFormatted = today.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });

            const rawActivities: Activity[] = await invoke('get_activities');

            // Filter for today
            const startOfToday = new Date(today.getFullYear(), today.getMonth(), today.getDate());

            activities = rawActivities.filter(act => {
                const actDate = new Date(act.start_time);
                return actDate >= startOfToday;
            });

            // Calculate total duration
            totalDurationToday = activities.reduce((acc, act) => acc + act.duration, 0);

            // Group by app
            const appUsage: Record<string, number> = {};
            for (const act of activities) {
                if (!appUsage[act.app_name]) appUsage[act.app_name] = 0;
                appUsage[act.app_name] += act.duration;
            }

            // Top 3 apps
            const sortedApps = Object.entries(appUsage)
                .map(([app_name, duration]) => ({ app_name, duration }))
                .sort((a, b) => b.duration - a.duration);

            const maxAppDuration = sortedApps.length > 0 ? sortedApps[0].duration : 1;
            topApps = sortedApps.slice(0, 3).map(app => ({
                ...app,
                percentage: Math.min(100, Math.round((app.duration / maxAppDuration) * 100))
            }));

            // Hourly Usage
            for (const act of activities) {
                const date = new Date(act.start_time);
                const hour = date.getHours();
                hourlyUsage[hour] += act.duration;
            }

            // Bin usage into 6 bins of 4 hours
            for (let i = 0; i < 24; i++) {
                const binIndex = Math.floor(i / 4);
                binnedUsage[binIndex] += hourlyUsage[i];
            }

            maxBinUsage = Math.max(...binnedUsage, 1); // min 1 to avoid /0

        } catch (error) {
            console.error("Failed to load activities:", error);
        }
    });
</script>

<TopBar title="Overview" subtitle={`Today, ${todayFormatted}`} />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-7xl mx-auto w-full">
    <!-- Bento Grid Layout -->
    <div class="grid grid-cols-12 gap-lg">
        <!-- Summary Card -->
        <div class="col-span-12 lg:col-span-4 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] flex flex-col justify-between">
            <div>
                <div class="flex items-center gap-sm mb-sm">
                    <span class="material-symbols-outlined text-primary">schedule</span>
                    <h2 class="font-label-md text-label-md text-on-surface-variant">Total Screen Time</h2>
                </div>
                <div class="font-display text-display text-on-surface mb-xs">{formatDuration(totalDurationToday)}</div>
                <div class="font-body-md text-body-md text-secondary flex items-center gap-xs">
                    <span class="material-symbols-outlined text-[16px]">info</span>
                    Includes tracked activities
                </div>
            </div>
            <div class="mt-xl">
                <div class="flex justify-between font-label-sm text-label-sm text-on-surface-variant mb-sm">
                    <span>Productivity Score</span>
                    <span class="text-on-surface font-semibold">78%</span>
                </div>
                <div class="h-2 w-full bg-surface-container-high rounded-full overflow-hidden">
                    <div class="h-full bg-primary rounded-full w-[78%]"></div>
                </div>
            </div>
        </div>

        <!-- Main Chart Card -->
        <div class="col-span-12 lg:col-span-8 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] flex flex-col">
            <div class="flex justify-between items-center mb-xl">
                <h2 class="font-headline-md text-headline-md text-on-surface">Daily Usage</h2>
                <div class="flex bg-surface-container-low p-xs rounded-lg">
                    <button class="px-md py-xs font-label-md text-label-md bg-surface-container-lowest shadow-sm rounded-md text-on-surface">Day</button>
                    <button class="px-md py-xs font-label-md text-label-md text-on-surface-variant hover:text-on-surface">Week</button>
                </div>
            </div>
            <!-- Bar Chart -->
            <div class="flex-1 flex items-end gap-md pt-lg h-[200px]">
                <!-- Bars -->
                {#each binnedUsage as usage, i}
                    {@const percentage = Math.max(5, (usage / maxBinUsage) * 100)}
                    {@const isMax = usage === maxBinUsage && usage > 0}
                    {@const timeLabels = ["4 AM", "8 AM", "12 PM", "4 PM", "8 PM", "12 AM"]}
                    <div class="flex-1 flex flex-col justify-end group h-full">
                        <div class="w-full rounded-t-full transition-colors relative {isMax ? 'bg-primary hover:bg-surface-tint shadow-sm' : 'bg-primary/20 hover:bg-primary/40'}" style="height: {percentage}%;">
                            <div class="absolute -top-10 left-1/2 -translate-x-1/2 bg-inverse-surface text-inverse-on-surface px-sm py-xs rounded font-label-sm text-label-sm opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap z-10">{formatDuration(usage)}</div>
                        </div>
                        <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm {isMax ? 'font-semibold' : ''}">{timeLabels[i]}</div>
                    </div>
                {/each}
            </div>
        </div>

        <!-- Most Used Apps -->
        <div class="col-span-12 lg:col-span-6 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)]">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-xl">Most Used</h2>
            <div class="flex flex-col gap-lg">
                {#if topApps.length === 0}
                    <div class="text-center text-on-surface-variant py-xl">No activity recorded yet today.</div>
                {:else}
                    {#each topApps as app, index}
                        <!-- App Item -->
                        <div>
                            <div class="flex justify-between items-center mb-sm">
                                <div class="flex items-center gap-sm">
                                    <div class="w-8 h-8 rounded-lg flex items-center justify-center {index === 0 ? 'bg-[#000000]' : index === 1 ? 'bg-[#0070eb]' : 'bg-[#4A154B]'}">
                                        <span class="material-symbols-outlined text-white text-[18px]">
                                            {index === 0 ? 'terminal' : index === 1 ? 'web' : 'apps'}
                                        </span>
                                    </div>
                                    <span class="font-body-md text-body-md text-on-surface truncate max-w-[150px]">{app.app_name || 'Unknown'}</span>
                                </div>
                                <span class="font-label-md text-label-md text-on-surface-variant">{formatDuration(app.duration)}</span>
                            </div>
                            <div class="h-2 w-full bg-surface-container-high rounded-full overflow-hidden">
                                <div class="h-full rounded-full transition-all duration-500 {index === 0 ? 'bg-primary' : index === 1 ? 'bg-[#0070eb] opacity-80' : 'bg-tertiary'}" style="width: {app.percentage}%;"></div>
                            </div>
                        </div>
                    {/each}
                {/if}
            </div>
        </div>

        <!-- Categories -->
        <div class="col-span-12 lg:col-span-6 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)]">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-xl">Categories</h2>
            <div class="flex gap-lg items-center h-[calc(100%-48px)]">
                <!-- Faux Donut Chart -->
                <div class="relative w-32 h-32 flex-shrink-0">
                    <svg class="w-full h-full transform -rotate-90" viewBox="0 0 36 36">
                        <path class="text-surface-container-high" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" stroke-width="4"></path>
                        <path class="text-primary" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" stroke-dasharray="60, 100" stroke-width="4"></path>
                        <path class="text-secondary" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" stroke-dasharray="25, 100" stroke-dashoffset="-60" stroke-width="4"></path>
                        <path class="text-tertiary" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" stroke-dasharray="15, 100" stroke-dashoffset="-85" stroke-width="4"></path>
                    </svg>
                </div>
                <!-- Legend -->
                <div class="flex-1 flex flex-col gap-md">
                    <div class="flex justify-between items-center">
                        <div class="flex items-center gap-sm">
                            <div class="w-3 h-3 rounded-full bg-primary"></div>
                            <span class="font-body-md text-body-md text-on-surface">Productivity</span>
                        </div>
                        <span class="font-label-md text-label-md text-on-surface-variant">60%</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <div class="flex items-center gap-sm">
                            <div class="w-3 h-3 rounded-full bg-secondary"></div>
                            <span class="font-body-md text-body-md text-on-surface">Communication</span>
                        </div>
                        <span class="font-label-md text-label-md text-on-surface-variant">25%</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <div class="flex items-center gap-sm">
                            <div class="w-3 h-3 rounded-full bg-tertiary"></div>
                            <span class="font-body-md text-body-md text-on-surface">Entertainment</span>
                        </div>
                        <span class="font-label-md text-label-md text-on-surface-variant">15%</span>
                    </div>
                </div>
            </div>
        </div>
    </div>
</main>
