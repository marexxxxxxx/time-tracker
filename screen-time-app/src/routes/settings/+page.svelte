<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
    import { settings } from '$lib/stores/settings';
    import { theme } from '$lib/stores/theme';
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';

    onMount(() => settings.load());

    function handleThemeChange(value: string) {
        settings.update('theme', value);
        theme.set(value as 'system' | 'light' | 'dark');
    }

    async function exportAllData() {
        try {
            const csv = await invoke<string>('export_activities_csv');
            const blob = new Blob([csv], { type: 'text/csv' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `screentime-full-export-${new Date().toISOString().split('T')[0]}.csv`;
            a.click();
            URL.revokeObjectURL(url);
        } catch (e) {
            console.error('Export failed:', e);
        }
    }

    async function clearAllData() {
        if (confirm('Are you sure? This will delete all tracked activities.')) {
            await invoke('clear_all_data');
            location.reload();
        }
    }

    async function resetDemoData() {
        if (confirm('Reset to demo data? This will replace all current data.')) {
            await invoke('reset_demo_data');
            location.reload();
        }
    }
</script>

<TopBar title="Settings" />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-3xl mx-auto w-full">
    <div class="space-y-xl">
        <!-- Appearance -->
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">Appearance</h2>

            <label class="block mb-md">
                <span class="font-label-md text-on-surface-variant mb-xs block">Theme</span>
                <div class="flex gap-sm">
                    {#each ['system', 'light', 'dark'] as option}
                        <button
                            class="px-md py-sm rounded-lg font-label-md transition-colors {$settings.theme === option ? 'bg-primary text-on-primary' : 'bg-surface-container-low text-on-surface-variant hover:bg-surface-container'}"
                            onclick={() => handleThemeChange(option)}
                        >
                            {option.charAt(0).toUpperCase() + option.slice(1)}
                        </button>
                    {/each}
                </div>
            </label>
        </section>

        <!-- Tracker -->
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">Tracker</h2>

            <label class="block mb-md">
                <span class="font-label-md text-on-surface-variant mb-xs block">Idle Detection Timeout</span>
                <select
                    value={$settings.idle_timeout}
                    onchange={(e) => settings.update('idle_timeout', (e.target as HTMLSelectElement).value)}
                    class="bg-surface-container-low border border-outline-variant/30 rounded-lg px-md py-sm font-body-md text-on-surface"
                >
                    <option value="5">5 minutes</option>
                    <option value="10">10 minutes</option>
                    <option value="15">15 minutes</option>
                    <option value="30">30 minutes</option>
                </select>
            </label>

            <label class="flex items-center justify-between py-sm">
                <span class="font-body-md text-on-surface">Pause Tracking</span>
                <button
                    aria-label="Toggle pause tracking"
                    class="w-12 h-6 rounded-full transition-colors relative {$settings.tracking_paused === 'true' ? 'bg-primary' : 'bg-outline-variant'}"
                    onclick={() => settings.update('tracking_paused', $settings.tracking_paused === 'true' ? 'false' : 'true')}
                >
                    <span class="absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform {$settings.tracking_paused === 'true' ? 'translate-x-6' : ''}"></span>
                </button>
            </label>
        </section>

        <!-- Notifications -->
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">Notifications</h2>

            <label class="flex items-center justify-between py-sm">
                <span class="font-body-md text-on-surface">Limit Warnings</span>
                <button
                    aria-label="Toggle limit warnings"
                    class="w-12 h-6 rounded-full transition-colors relative {$settings.limit_warnings === 'true' ? 'bg-primary' : 'bg-outline-variant'}"
                    onclick={() => settings.update('limit_warnings', $settings.limit_warnings === 'true' ? 'false' : 'true')}
                >
                    <span class="absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform {$settings.limit_warnings === 'true' ? 'translate-x-6' : ''}"></span>
                </button>
            </label>

            <label class="flex items-center justify-between py-sm">
                <span class="font-body-md text-on-surface">Daily Summary</span>
                <button
                    aria-label="Toggle daily summary"
                    class="w-12 h-6 rounded-full transition-colors relative {$settings.daily_summary === 'true' ? 'bg-primary' : 'bg-outline-variant'}"
                    onclick={() => settings.update('daily_summary', $settings.daily_summary === 'true' ? 'false' : 'true')}
                >
                    <span class="absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform {$settings.daily_summary === 'true' ? 'translate-x-6' : ''}"></span>
                </button>
            </label>
        </section>

        <!-- Data Management -->
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">Data Management</h2>

            <div class="flex flex-col gap-sm">
                <button class="bg-primary/5 text-primary hover:bg-primary/10 px-md py-sm rounded-lg font-label-md text-label-md transition-colors text-left flex items-center gap-sm" onclick={exportAllData}>
                    <span class="material-symbols-outlined text-[18px]">download</span>
                    Export All Data (CSV)
                </button>
                <button class="bg-error/5 text-error hover:bg-error/10 px-md py-sm rounded-lg font-label-md text-label-md transition-colors text-left flex items-center gap-sm" onclick={clearAllData}>
                    <span class="material-symbols-outlined text-[18px]">delete</span>
                    Clear All Data
                </button>
                <button class="bg-surface-container-low text-on-surface-variant hover:bg-surface-container px-md py-sm rounded-lg font-label-md text-label-md transition-colors text-left flex items-center gap-sm" onclick={resetDemoData}>
                    <span class="material-symbols-outlined text-[18px]">refresh</span>
                    Reset Demo Data
                </button>
            </div>
        </section>

        <!-- About -->
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-lg">About</h2>
            <div class="space-y-sm font-body-md text-on-surface-variant">
                <p><strong class="text-on-surface">Screen Time Tracker</strong> v1.0.0</p>
                <p>Track your digital wellness. All data stays local on your device.</p>
            </div>
        </section>
    </div>
</main>
