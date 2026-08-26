<script lang="ts">
    import { formatDuration } from '$lib/stores/activities';

    let { items, totalDuration }: {
        items: Array<{ name: string; duration: number; color: string; icon: string }>;
        totalDuration: number;
    } = $props();

    function pct(dur: number): number {
        if (totalDuration === 0) return 0;
        return Math.round((dur / totalDuration) * 100);
    }
</script>

<h2 class="font-headline-md text-headline-md text-on-surface mb-xl">Most Used</h2>
<div class="flex flex-col gap-lg">
    {#each items as item}
        <div>
            <div class="flex justify-between items-center mb-sm">
                <div class="flex items-center gap-sm">
                    <div class="w-8 h-8 rounded-lg flex items-center justify-center" style="background-color: {item.color}">
                        <span class="material-symbols-outlined text-white text-[18px]">{item.icon}</span>
                    </div>
                    <span class="font-body-md text-body-md text-on-surface">{item.name}</span>
                </div>
                <span class="font-label-md text-label-md text-on-surface-variant">{formatDuration(item.duration)}</span>
            </div>
            <div class="h-2 w-full bg-surface-container-high rounded-full overflow-hidden">
                <div class="h-full rounded-full" style="width: {pct(item.duration)}%; background-color: {item.color}"></div>
            </div>
        </div>
    {/each}
</div>
