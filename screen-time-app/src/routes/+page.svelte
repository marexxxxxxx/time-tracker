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
