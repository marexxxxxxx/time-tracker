<script lang="ts">
    let {
        open = false,
        appName = "",
        dailyLimit = 0,
        weeklyLimit = 0,
        limitEnabled = false,
        onsave,
        onclose
    }: {
        open: boolean;
        appName: string;
        dailyLimit: number;
        weeklyLimit: number;
        limitEnabled: boolean;
        onsave: (daily: number, weekly: number, enabled: boolean) => void;
        onclose: () => void;
    } = $props();

    let daily = $state(dailyLimit);
    let weekly = $state(weeklyLimit);
    let enabled = $state(limitEnabled);

    $effect(() => {
        console.log('[LimitEditor] effect: open=', open, 'appName=', appName);
        if (open) {
            daily = dailyLimit;
            weekly = weeklyLimit;
            enabled = limitEnabled;
        }
    });
</script>

{#if open}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40" onclick={onclose} role="presentation">
        <div
            class="bg-surface-container-lowest rounded-2xl shadow-xl w-full max-w-md mx-4 overflow-hidden"
            onclick={(e) => e.stopPropagation()}
            role="dialog"
            aria-label="Edit limits for {appName}"
        >
            <div class="px-xl py-lg border-b border-outline-variant/20">
                <h3 class="font-title-lg text-title-lg text-on-surface">Limits for {appName}</h3>
            </div>

            <div class="px-xl py-lg space-y-lg">
                <label class="flex items-center justify-between cursor-pointer">
                    <span class="font-body-md text-body-md text-on-surface">Enable Time Limits</span>
                    <button
                        class="relative inline-block w-12 align-middle select-none transition duration-200 ease-in"
                        onclick={() => enabled = !enabled}
                        aria-label="Toggle limits"
                    >
                        <div class="block overflow-hidden h-6 rounded-full transition-colors duration-200 ease-in-out {enabled ? 'bg-primary' : 'bg-surface-variant'}">
                            <div class="absolute top-[2px] left-[2px] w-5 h-5 bg-white rounded-full shadow-sm transition-transform duration-200 ease-in-out {enabled ? 'translate-x-[24px]' : 'translate-x-0'}"></div>
                        </div>
                    </button>
                </label>

                {#if enabled}
                    <div class="space-y-md">
                        <div>
                            <label for="daily-limit" class="font-label-sm text-label-sm text-on-surface-variant block mb-xs">Daily Limit (minutes)</label>
                            <input
                                id="daily-limit"
                                type="number"
                                min="0"
                                step="5"
                                bind:value={daily}
                                class="w-full bg-surface-container-high text-on-surface px-md py-sm rounded-lg border border-outline-variant/40 focus:border-primary focus:outline-none font-body-md"
                                placeholder="0 = no limit"
                            />
                        </div>
                        <div>
                            <label for="weekly-limit" class="font-label-sm text-label-sm text-on-surface-variant block mb-xs">Weekly Limit (minutes)</label>
                            <input
                                id="weekly-limit"
                                type="number"
                                min="0"
                                step="5"
                                bind:value={weekly}
                                class="w-full bg-surface-container-high text-on-surface px-md py-sm rounded-lg border border-outline-variant/40 focus:border-primary focus:outline-none font-body-md"
                                placeholder="0 = no limit"
                            />
                        </div>
                    </div>
                {/if}
            </div>

            <div class="px-xl py-md border-t border-outline-variant/20 flex justify-end gap-sm">
                <button
                    class="px-md py-sm rounded-lg font-label-md text-label-md text-on-surface hover:bg-surface-container-low transition-colors"
                    onclick={onclose}
                >
                    Cancel
                </button>
                <button
                    class="bg-primary text-on-primary px-md py-sm rounded-lg font-label-md text-label-md hover:bg-primary-container transition-colors"
                    onclick={() => onsave(daily, weekly, enabled)}
                >
                    Save
                </button>
            </div>
        </div>
    </div>
{/if}
