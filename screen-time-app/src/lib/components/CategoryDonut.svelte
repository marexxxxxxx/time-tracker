<script lang="ts">
    import { onMount } from 'svelte';
    import DonutChart from './DonutChart.svelte';

    let { categories }: {
        categories: Array<{ name: string; percentage: number; color: string }>;
    } = $props();

    let version = $state(0);

    function resolveColor(color: string): string {
        if (color.startsWith('--')) {
            return getComputedStyle(document.documentElement).getPropertyValue(color).trim();
        }
        return color;
    }

    onMount(() => {
        const observer = new MutationObserver(() => { version++; });
        observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] });
        return () => observer.disconnect();
    });
</script>

<h2 class="font-headline-md text-headline-md text-on-surface mb-xl">Categories</h2>
<div class="flex gap-lg items-center">
    <DonutChart
        labels={categories.map(c => c.name)}
        data={categories.map(c => c.percentage)}
        colors={categories.map(c => c.color)}
    />
    <div class="flex-1 flex flex-col gap-md">
        {#each categories as cat (cat.name)}
            <div class="flex justify-between items-center">
                <div class="flex items-center gap-sm">
                    {#key version}
                        <div class="w-3 h-3 rounded-full" style="background-color: {resolveColor(cat.color)}"></div>
                    {/key}
                    <span class="font-body-md text-body-md text-on-surface">{cat.name}</span>
                </div>
                <span class="font-label-md text-label-md text-on-surface-variant">{cat.percentage}%</span>
            </div>
        {/each}
    </div>
</div>
