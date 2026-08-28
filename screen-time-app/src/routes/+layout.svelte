<script lang="ts">
    import Sidebar from '$lib/components/Sidebar.svelte';
    import { onMount, onDestroy } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { isIdle, parseIdleEvent } from '$lib/stores/idle';
    import { fetchActivities, fetchDailySummary, fetchProductivityByWeek, fetchDeepWorkSessions } from '$lib/stores/activities';
    import '../app.css';

    let { children, data } = $props();
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

    let lastEventId = 0;

    type TrackedEvent = { id: number; event_type: string; payload: string };

    async function pollEvents() {
        try {
            const events = await invoke<TrackedEvent[]>('poll_events', { afterId: lastEventId });
            for (const ev of events) {
                if (ev.event_type === 'limit-warning') {
                    const p = JSON.parse(ev.payload);
                    showWarning(p.app_name, p.limit_type, p.remaining_minutes);
                } else if (ev.event_type === 'idle-state') {
                    const idle = parseIdleEvent(ev.payload);
                    if (idle) isIdle.set(idle.is_idle);
                }
                lastEventId = ev.id;
            }
        } catch (e) {
            console.error('Failed to poll events:', e);
        }
    }

    onMount(async () => {
        try {
            pollEvents();
            await fetchAll();
            pollInterval = setInterval(() => {
                pollEvents();
                fetchAll();
            }, 5000);
        } catch (e) {
            console.error("Failed to setup layout:", e);
        }
    });

    onDestroy(() => {
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
