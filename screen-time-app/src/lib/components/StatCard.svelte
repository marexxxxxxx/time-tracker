<script lang="ts">
    let {
        icon,
        label,
        value,
        subtext = "",
        progress = -1,
        progressColor = "bg-primary"
    }: {
        icon: string;
        label: string;
        value: string;
        subtext?: string;
        progress?: number;
        progressColor?: string;
    } = $props();
</script>

<div class="bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] dark:bg-surface-container dark:border-outline/20 flex flex-col justify-between">
    <div>
        <div class="flex items-center gap-sm mb-sm">
            <span class="material-symbols-outlined text-primary">{icon}</span>
            <h2 class="font-label-md text-label-md text-on-surface-variant">{label}</h2>
        </div>
        <div class="font-display text-display text-on-surface">{value}</div>
        {#if subtext}
            <div class="font-body-md text-body-md text-on-surface-variant flex items-center gap-xs mt-xs">
                {subtext}
            </div>
        {/if}
    </div>
    {#if progress >= 0}
        <div class="mt-xl">
            <div class="h-2 w-full bg-surface-container-high rounded-full overflow-hidden">
                <div class="h-full {progressColor} rounded-full" style="width: {progress}%"></div>
            </div>
        </div>
    {/if}
</div>
