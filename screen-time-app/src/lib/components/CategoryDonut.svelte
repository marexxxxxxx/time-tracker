<script lang="ts">
    let { categories }: {
        categories: Array<{ name: string; percentage: number; color: string }>;
    } = $props();

    let segments = $derived(
        categories.reduce((acc, cat, i) => {
            const offset = acc.length > 0 ? acc[acc.length - 1].offset + acc[acc.length - 1].percentage : 0;
            acc.push({ ...cat, offset });
            return acc;
        }, [] as Array<{ name: string; percentage: number; color: string; offset: number }>)
    );
</script>

<h2 class="font-headline-md text-headline-md text-on-surface mb-xl">Categories</h2>
<div class="flex gap-lg items-center">
    <div class="relative w-32 h-32 flex-shrink-0">
        <svg class="w-full h-full transform -rotate-90" viewBox="0 0 36 36">
            <path class="text-surface-container-high" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" stroke-width="4"></path>
            {#each segments as seg}
                <path
                    d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                    fill="none"
                    stroke={seg.color}
                    stroke-dasharray="{seg.percentage}, 100"
                    stroke-dashoffset="-{seg.offset}"
                    stroke-width="4"
                ></path>
            {/each}
        </svg>
    </div>
    <div class="flex-1 flex flex-col gap-md">
        {#each categories as cat}
            <div class="flex justify-between items-center">
                <div class="flex items-center gap-sm">
                    <div class="w-3 h-3 rounded-full" style="background-color: {cat.color}"></div>
                    <span class="font-body-md text-body-md text-on-surface">{cat.name}</span>
                </div>
                <span class="font-label-md text-label-md text-on-surface-variant">{cat.percentage}%</span>
            </div>
        {/each}
    </div>
</div>
