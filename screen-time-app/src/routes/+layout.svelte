<script lang="ts">
    import Sidebar from '$lib/components/Sidebar.svelte';
    import { onMount, onDestroy } from 'svelte';
    import { setupIdleListener } from '$lib/stores/idle';
    import { fetchActivities } from '$lib/stores/activities';
    import '../app.css';

    let { children, data } = $props();

    let unlisten: (() => void) | undefined;
    let pollInterval: ReturnType<typeof setInterval>;

    onMount(async () => {
        try {
            unlisten = await setupIdleListener();
            // Fetch initial data
            await fetchActivities();
            // Poll for fresh data periodically since background tracker runs
            pollInterval = setInterval(fetchActivities, 5000);
        } catch (e) {
            console.error("Failed to setup layout:", e);
        }
    });

    onDestroy(() => {
        if (unlisten) {
            unlisten();
        }
        if (pollInterval) {
            clearInterval(pollInterval);
        }
    });
</script>

<div class="flex overflow-hidden">
    <Sidebar />
    <div class="flex-1 ml-[280px] flex flex-col h-screen overflow-hidden bg-background">
        <main class="flex-1 overflow-y-auto w-full">
            {@render children()}
        </main>
    </div>
</div>