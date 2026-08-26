<script lang="ts">
    import Sidebar from '$lib/components/Sidebar.svelte';
    import { onMount, onDestroy } from 'svelte';
    import { listen } from '@tauri-apps/api/event';
    import { setupIdleListener } from '$lib/stores/idle';
    import { fetchActivities, fetchDailySummary, fetchProductivityByWeek, fetchDeepWorkSessions } from '$lib/stores/activities';
    import '../app.css';

    let { children, data } = $props();
    let unlisten: (() => void) | undefined;
    let unlistenWarning: (() => void) | undefined;
    let pollInterval: ReturnType<typeof setInterval>;

    let warning = $state<{ appName: string; limitType: string; remainingMinutes: number } | null>(null);
    let warningTimeout: ReturnType<typeof setTimeout> | null = null;

    async function fetchAll() {
        await Promise.all([
            fetchActivities(),
            fetchDailySummary(),
            fetchProductivityByWeek(),
            fetchDeepWorkSessions(),
        ]);
    }

    function showWarning(appName: string, limitType: string, remainingMinutes: number) {
        warning = { appName, limitType, remainingMinutes };
        if (warningTimeout) clearTimeout(warningTimeout);
        warningTimeout = setTimeout(() => { warning = null; }, 10000);
    }

    onMount(async () => {
        try {
            unlisten = await setupIdleListener();
            unlistenWarning = await listen<{ app_name: string; limit_type: string; remaining_minutes: number }>('limit-warning', (event) => {
                showWarning(event.payload.app_name, event.payload.limit_type, event.payload.remaining_minutes);
            });
            await fetchAll();
            pollInterval = setInterval(fetchAll, 5000);
        } catch (e) {
            console.error("Failed to setup layout:", e);
        }
    });

    onDestroy(() => {
        if (unlisten) unlisten();
        if (unlistenWarning) unlistenWarning();
        if (pollInterval) clearInterval(pollInterval);
        if (warningTimeout) clearTimeout(warningTimeout);
    });
</script>

<div class="flex min-h-screen">
    <Sidebar />
    <div class="flex-1 ml-[280px] flex flex-col min-h-screen">
        {@render children()}
    </div>
</div>

{#if warning}
    <div class="fixed bottom-6 right-6 z-50 bg-tertiary-container text-on-tertiary-container px-lg py-md rounded-xl shadow-lg flex items-center gap-md max-w-sm animate-slide-up">
        <span class="material-symbols-outlined text-tertiary">warning</span>
        <div>
            <p class="font-body-sm text-body-sm font-medium">{warning.appName} — {warning.limitType} limit</p>
            <p class="font-label-sm text-label-sm text-on-tertiary-container/80">~{warning.remainingMinutes}m remaining before block</p>
        </div>
        <button
            class="ml-auto p-xs rounded-lg hover:bg-tertiary/10 transition-colors"
            onclick={() => warning = null}
            aria-label="Dismiss warning"
        >
            <span class="material-symbols-outlined text-[18px]">close</span>
        </button>
    </div>
{/if}
