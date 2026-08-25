<script lang="ts">
    let { sessions }: {
        sessions: Array<{
            title: string;
            startTime: string;
            endTime: string;
            duration: string;
            category: string;
            color: string;
        }>;
    } = $props();
</script>

<h3 class="font-headline-md text-headline-md text-on-surface font-semibold mb-lg">Recent Deep Work</h3>
<div class="space-y-sm">
    {#each sessions as session}
        <div class="flex items-center justify-between p-sm hover:bg-surface-container-low dark:hover:bg-surface-container-highest/20 rounded-lg transition-colors border border-transparent hover:border-outline-variant/30 dark:hover:border-outline/20">
            <div class="flex items-center gap-md">
                <div class="w-2 h-10 rounded-full" style="background-color: {session.color}"></div>
                <div>
                    <p class="font-body-md text-body-md font-medium text-on-surface">{session.title}</p>
                    <p class="font-label-sm text-label-sm text-on-surface-variant flex items-center gap-xs">
                        <span class="material-symbols-outlined text-[14px]">schedule</span>
                        {session.startTime} - {session.endTime}
                    </p>
                </div>
            </div>
            <div class="text-right">
                <p class="font-headline-md text-body-md font-semibold" style="color: {session.color}">{session.duration}</p>
                <span class="inline-block px-2 py-0.5 rounded-md bg-surface-variant dark:bg-surface-container-highest text-on-surface-variant font-label-sm text-[10px] uppercase">{session.category}</span>
            </div>
        </div>
    {/each}
</div>
