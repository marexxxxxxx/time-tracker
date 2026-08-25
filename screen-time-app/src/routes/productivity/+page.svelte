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
