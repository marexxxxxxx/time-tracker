<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
    import PageHeader from '$lib/components/PageHeader.svelte';
    import AppBlockCard from '$lib/components/AppBlockCard.svelte';

    let blockedApps = $state([
        { appName: "Instagram", icon: "photo_camera", iconBg: "rgba(225,48,108,0.1)", category: "Social Media", limit: "Limit: 30m / day", isBlocked: true, usage: "24m used", usagePct: 80 },
        { appName: "TikTok", icon: "music_note", iconBg: "rgba(0,0,0,0.05)", category: "Social Media", limit: "Blocked entirely", isBlocked: true },
        { appName: "Twitter", icon: "chat_bubble", iconBg: "rgba(29,161,242,0.1)", category: "Social Media", limit: "Limit: 1h / day", isBlocked: false, usage: "6m used", usagePct: 10 },
        { appName: "YouTube", icon: "play_arrow", iconBg: "rgba(255,0,0,0.1)", category: "Entertainment", limit: "Limit: 2h / day", isBlocked: true },
        { appName: "Netflix", icon: "movie", iconBg: "rgba(229,9,20,0.1)", category: "Entertainment", limit: "Limit: 1h 30m / day", isBlocked: true },
    ]);

    function toggleApp(index: number) {
        blockedApps[index].isBlocked = !blockedApps[index].isBlocked;
    }

    function appsForCategory(cat: string) {
        return blockedApps
            .map((a, i) => ({ ...a, index: i }))
            .filter(a => a.category === cat);
    }

    const socialApps = $derived(appsForCategory("Social Media"));
    const entertainmentApps = $derived(appsForCategory("Entertainment"));
</script>

<TopBar title="Screen Time" />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-7xl mx-auto w-full">
    <div class="max-w-4xl mx-auto space-y-xl">
        <PageHeader title="Blocked Apps & Limits" description="Manage restrictions and focus schedules for your applications." />

        <!-- Social Media -->
        <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
            <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Social Media</h3>
                <span class="font-label-sm text-label-sm text-on-surface-variant bg-surface-variant/50 px-sm py-xs rounded-md">{socialApps.length} Apps</span>
            </div>
            <div class="divide-y divide-outline-variant/20">
                {#each socialApps as app}
                    <AppBlockCard
                        appName={app.appName}
                        icon={app.icon}
                        iconBg={app.iconBg}
                        limit={app.limit}
                        isBlocked={app.isBlocked}
                        usage={app.usage}
                        usagePct={app.usagePct}
                        onToggle={() => toggleApp(app.index)}
                    />
                {/each}
            </div>
        </section>

        <!-- Entertainment -->
        <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
            <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Entertainment</h3>
                <span class="font-label-sm text-label-sm text-on-surface-variant bg-surface-variant/50 px-sm py-xs rounded-md">{entertainmentApps.length} Apps</span>
            </div>
            <div class="divide-y divide-outline-variant/20">
                {#each entertainmentApps as app}
                    <AppBlockCard
                        appName={app.appName}
                        icon={app.icon}
                        iconBg={app.iconBg}
                        limit={app.limit}
                        isBlocked={app.isBlocked}
                        usage={app.usage}
                        usagePct={app.usagePct}
                        onToggle={() => toggleApp(app.index)}
                    />
                {/each}
            </div>
        </section>
    </div>
</main>
