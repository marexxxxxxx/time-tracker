<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
    import PageHeader from '$lib/components/PageHeader.svelte';
    import AppBlockCard from '$lib/components/AppBlockCard.svelte';
    import AddAppModal from '$lib/components/AddAppModal.svelte';
    import LimitEditor from '$lib/components/LimitEditor.svelte';
    import { blockedApps, toggleBlockedApp, removeBlockedApp, fetchBlockedApps, updateAppLimits } from '$lib/stores/blockedApps';
    import { onMount } from 'svelte';

    let showModal = $state(false);
    let editingApp = $state<{ id: number; name: string; daily: number; weekly: number; enabled: boolean } | null>(null);

    onMount(() => {
        fetchBlockedApps();
    });

    function categoryForApp(appName: string): string {
        const social = ['instagram', 'tiktok', 'twitter', 'facebook', 'reddit', 'discord', 'slack'];
        const entertainment = ['youtube', 'netflix', 'spotify', 'twitch', 'steam'];
        const lower = appName.toLowerCase();
        if (social.some(s => lower.includes(s))) return 'Social Media';
        if (entertainment.some(e => lower.includes(e))) return 'Entertainment';
        return 'Other';
    }

    function openLimitEditor(app: { id: number; app_name: string; daily_limit_minutes: number; weekly_limit_minutes: number; limit_enabled: boolean }) {
        console.log('[LimitEditor] openLimitEditor called for', app.app_name, app);
        editingApp = {
            id: app.id,
            name: app.app_name,
            daily: app.daily_limit_minutes,
            weekly: app.weekly_limit_minutes,
            enabled: app.limit_enabled,
        };
    }

    function handleSaveLimits(daily: number, weekly: number, enabled: boolean) {
        if (editingApp) {
            updateAppLimits(editingApp.id, daily, weekly, enabled);
        }
        editingApp = null;
    }

    let socialApps = $derived(
        $blockedApps.filter(a => categoryForApp(a.app_name) === 'Social Media')
    );
    let entertainmentApps = $derived(
        $blockedApps.filter(a => categoryForApp(a.app_name) === 'Entertainment')
    );
    let otherApps = $derived(
        $blockedApps.filter(a => categoryForApp(a.app_name) === 'Other')
    );
</script>

<TopBar title="Screen Time" />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-7xl mx-auto w-full">
    <div class="max-w-4xl mx-auto space-y-xl">
        <PageHeader title="Blocked Apps & Limits" description="Manage restrictions and focus schedules for your applications." />

        <div class="flex justify-end mb-lg">
            <button
                class="bg-primary text-on-primary px-md py-sm rounded-lg font-label-md text-label-md hover:bg-primary-container transition-colors flex items-center gap-xs"
                onclick={() => showModal = true}
            >
                <span class="material-symbols-outlined text-[18px]">add</span>
                Block App
            </button>
        </div>

        {#if $blockedApps.length === 0}
            <div class="text-center py-xxl">
                <p class="text-on-surface-variant font-body-md">No blocked apps configured. Add apps to start blocking.</p>
            </div>
        {/if}

        {#if socialApps.length > 0}
            <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
                <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                    <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Social Media</h3>
                    <span class="font-label-sm text-label-sm text-on-surface-variant bg-surface-variant/50 px-sm py-xs rounded-md">{socialApps.length} Apps</span>
                </div>
                <div class="divide-y divide-outline-variant/20">
                    {#each socialApps as app}
                        <AppBlockCard
                            appName={app.app_name}
                            icon="smart_display"
                            iconBg="rgba(0,0,0,0.05)"
                            limit={app.is_blocked ? "Blocked" : "Allowed"}
                            isBlocked={app.is_blocked}
                            dailyLimit={app.daily_limit_minutes}
                            weeklyLimit={app.weekly_limit_minutes}
                            limitEnabled={app.limit_enabled}
                            onToggle={() => toggleBlockedApp(app.id)}
                            onRemove={() => removeBlockedApp(app.id)}
                            onEditLimits={() => openLimitEditor(app)}
                        />
                    {/each}
                </div>
            </section>
        {/if}

        {#if entertainmentApps.length > 0}
            <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
                <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                    <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Entertainment</h3>
                    <span class="font-label-sm text-label-sm text-on-surface-variant bg-surface-variant/50 px-sm py-xs rounded-md">{entertainmentApps.length} Apps</span>
                </div>
                <div class="divide-y divide-outline-variant/20">
                    {#each entertainmentApps as app}
                        <AppBlockCard
                            appName={app.app_name}
                            icon="play_arrow"
                            iconBg="rgba(255,0,0,0.1)"
                            limit={app.is_blocked ? "Blocked" : "Allowed"}
                            isBlocked={app.is_blocked}
                            dailyLimit={app.daily_limit_minutes}
                            weeklyLimit={app.weekly_limit_minutes}
                            limitEnabled={app.limit_enabled}
                            onToggle={() => toggleBlockedApp(app.id)}
                            onRemove={() => removeBlockedApp(app.id)}
                            onEditLimits={() => openLimitEditor(app)}
                        />
                    {/each}
                </div>
            </section>
        {/if}

        {#if otherApps.length > 0}
            <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
                <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                    <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Other</h3>
                    <span class="font-label-sm text-label-sm text-on-surface-variant bg-surface-variant/50 px-sm py-xs rounded-md">{otherApps.length} Apps</span>
                </div>
                <div class="divide-y divide-outline-variant/20">
                    {#each otherApps as app}
                        <AppBlockCard
                            appName={app.app_name}
                            icon="apps"
                            iconBg="rgba(0,0,0,0.05)"
                            limit={app.is_blocked ? "Blocked" : "Allowed"}
                            isBlocked={app.is_blocked}
                            dailyLimit={app.daily_limit_minutes}
                            weeklyLimit={app.weekly_limit_minutes}
                            limitEnabled={app.limit_enabled}
                            onToggle={() => toggleBlockedApp(app.id)}
                            onRemove={() => removeBlockedApp(app.id)}
                            onEditLimits={() => openLimitEditor(app)}
                        />
                    {/each}
                </div>
            </section>
        {/if}
    </div>

    <AddAppModal open={showModal} onclose={() => showModal = false} />

    <LimitEditor
        open={editingApp !== null}
        appName={editingApp?.name ?? ''}
        dailyLimit={editingApp?.daily ?? 0}
        weeklyLimit={editingApp?.weekly ?? 0}
        limitEnabled={editingApp?.enabled ?? false}
        onsave={handleSaveLimits}
        onclose={() => { editingApp = null; }}
    />
</main>
