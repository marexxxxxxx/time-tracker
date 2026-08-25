<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
    import PageHeader from '$lib/components/PageHeader.svelte';
    import AppBlockCard from '$lib/components/AppBlockCard.svelte';
    import { blockedApps, toggleBlockedApp, fetchBlockedApps } from '$lib/stores/blockedApps';
    import { onMount } from 'svelte';

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
                            onToggle={() => toggleBlockedApp(app.id)}
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
                            onToggle={() => toggleBlockedApp(app.id)}
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
                            onToggle={() => toggleBlockedApp(app.id)}
                        />
                    {/each}
                </div>
            </section>
        {/if}
    </div>
</main>
