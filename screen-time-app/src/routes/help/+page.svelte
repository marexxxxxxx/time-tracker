<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
</script>

<TopBar title="Help" />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-3xl mx-auto w-full">
    <div class="space-y-xl">
        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-md">How Tracking Works</h2>
            <div class="font-body-md text-on-surface-variant space-y-sm">
                <p>Screen Time Tracker monitors your active window every 3 seconds to understand how you spend your time on the computer.</p>
                <p>Apps are automatically categorized as Coding, Design, Communication, Entertainment, or Neutral. Browser tabs are tracked by site name.</p>
            </div>
        </section>

        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-md">Blocked Apps & Limits</h2>
            <div class="font-body-md text-on-surface-variant space-y-sm">
                <p>You can block specific apps or set daily/weekly time limits. When a limit is reached, the app will be closed automatically.</p>
                <p>Go to <strong class="text-on-surface">Blocked Apps</strong> in the sidebar to manage restrictions.</p>
            </div>
        </section>

        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-md">Data Privacy</h2>
            <div class="font-body-md text-on-surface-variant space-y-sm">
                <p>All data is stored locally in an SQLite database on your device. Nothing is sent to the cloud.</p>
                <p>You can export or delete all data at any time from <strong class="text-on-surface">Settings</strong>.</p>
            </div>
        </section>

        <section class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-md">Support</h2>
            <div class="font-body-md text-on-surface-variant space-y-sm">
                <p>Found a bug or have a feature request?</p>
                <a href="https://github.com/marexxxxxxx/time-tracker/issues" class="text-primary hover:underline inline-flex items-center gap-xs">
                    <span class="material-symbols-outlined text-[18px]">open_in_new</span>
                    Open an issue on GitHub
                </a>
            </div>
        </section>
    </div>
</main>
