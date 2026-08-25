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
