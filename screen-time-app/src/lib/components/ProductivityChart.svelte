<script lang="ts">
    let { data }: {
        data: Array<{ day: string; productive: number; neutral: number; leisure: number }>;
    } = $props();
</script>

<h3 class="font-headline-md text-headline-md text-on-surface font-semibold mb-lg">Work vs. Leisure</h3>
<div class="flex items-end gap-sm h-48 mt-md border-b border-outline-variant/30 dark:border-outline/20 pb-sm">
    {#each data as d}
        <div class="flex-1 flex flex-col justify-end group">
            <div class="w-full bg-tertiary rounded-t-full transition-all opacity-80 group-hover:opacity-100" style="height: {d.leisure}%"></div>
            <div class="w-full bg-surface-variant dark:bg-surface-container-highest transition-all opacity-80 group-hover:opacity-100" style="height: {d.neutral}%"></div>
            <div class="w-full bg-primary transition-all opacity-80 group-hover:opacity-100 rounded-b-sm" style="height: {d.productive}%"></div>
            <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm">{d.day}</div>
        </div>
    {/each}
</div>
