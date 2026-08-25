<script lang="ts">
    import { addBlockedApp } from '$lib/stores/blockedApps';

    let { open = false, onclose }: { open: boolean; onclose: () => void } = $props();
    let appName = $state('');

    async function handleSubmit() {
        if (appName.trim()) {
            await addBlockedApp(appName.trim());
            appName = '';
            onclose();
        }
    }
</script>

{#if open}
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div class="fixed inset-0 z-[100] flex items-center justify-center bg-black/40" onclick={onclose}>
        <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
        <div class="bg-surface-container-lowest dark:bg-surface-container border border-outline-variant/30 dark:border-outline/20 rounded-2xl p-xl w-full max-w-md shadow-xl" onclick={(e) => e.stopPropagation()}>
            <h3 class="font-headline-md text-headline-md text-on-surface mb-lg">Block an App</h3>
            <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
                <label class="font-label-md text-label-md text-on-surface-variant block mb-sm">Application Name</label>
                <input
                    type="text"
                    bind:value={appName}
                    placeholder="e.g. Instagram, YouTube"
                    class="w-full px-md py-sm rounded-lg border border-outline-variant bg-surface text-on-surface font-body-md focus:outline-none focus:border-primary"
                />
                <div class="flex justify-end gap-sm mt-lg">
                    <button type="button" class="px-md py-sm rounded-lg font-label-md text-label-md text-on-surface-variant hover:bg-surface-container-low" onclick={onclose}>Cancel</button>
                    <button type="submit" class="px-md py-sm rounded-lg font-label-md text-label-md text-on-primary bg-primary hover:bg-primary-container">Block</button>
                </div>
            </form>
        </div>
    </div>
{/if}
