<script lang="ts">
    import { blockedApps, fetchBlockedApps, updateAppLimits } from '$lib/stores/blockedApps';
    import { onMount } from 'svelte';

    let { open, onclose }: { open: boolean; onclose: () => void } = $props();
    let selectedApp = $state('');
    let dailyLimit = $state(60);
    let weeklyLimit = $state(300);

    onMount(() => fetchBlockedApps());

    function save() {
        const app = $blockedApps.find(a => a.app_name === selectedApp);
        if (app) {
            updateAppLimits(app.id, dailyLimit, weeklyLimit, true);
        }
        onclose();
    }
</script>

{#if open}
    <div class="fixed inset-0 z-50 bg-black/40 flex items-center justify-center" onclick={onclose} onkeydown={(e) => e.key === 'Escape' && onclose()}>
        <div class="bg-surface-container-high border border-outline-variant/30 rounded-2xl shadow-xl p-xl w-[400px]" onclick={(e) => e.stopPropagation()}>
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">Add Time Limit</h2>

            <label class="block mb-md">
                <span class="font-label-md text-on-surface-variant mb-xs block">App</span>
                <select bind:value={selectedApp} class="w-full bg-surface-container-low border border-outline-variant/30 rounded-lg px-md py-sm font-body-md text-on-surface">
                    <option value="">Select an app...</option>
                    {#each $blockedApps as app}
                        <option value={app.app_name}>{app.app_name}</option>
                    {/each}
                </select>
            </label>

            <label class="block mb-md">
                <span class="font-label-md text-on-surface-variant mb-xs block">Daily Limit (minutes)</span>
                <input type="number" bind:value={dailyLimit} min="0" class="w-full bg-surface-container-low border border-outline-variant/30 rounded-lg px-md py-sm font-body-md text-on-surface" />
            </label>

            <label class="block mb-lg">
                <span class="font-label-md text-on-surface-variant mb-xs block">Weekly Limit (minutes)</span>
                <input type="number" bind:value={weeklyLimit} min="0" class="w-full bg-surface-container-low border border-outline-variant/30 rounded-lg px-md py-sm font-body-md text-on-surface" />
            </label>

            <div class="flex justify-end gap-sm">
                <button class="px-md py-sm rounded-lg font-label-md text-on-surface-variant hover:bg-surface-container-low" onclick={onclose}>Cancel</button>
                <button class="px-md py-sm rounded-lg font-label-md text-on-primary bg-primary hover:bg-primary-container" onclick={save} disabled={!selectedApp}>Save</button>
            </div>
        </div>
    </div>
{/if}
