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
