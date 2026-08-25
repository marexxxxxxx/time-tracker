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
