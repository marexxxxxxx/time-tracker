<script lang="ts">
    import ThemeToggle from './ThemeToggle.svelte';
    let { title = "Screen Time", subtitle = "" }: { title?: string; subtitle?: string } = $props();
</script>

<header class="fixed top-0 right-0 left-[280px] z-40 bg-surface/60 dark:bg-inverse-surface/60 backdrop-blur-2xl border-b border-outline-variant/20 dark:border-outline/20 shadow-none flex items-center justify-between px-margin-desktop py-lg w-[calc(100%-280px)] h-[88px]">
    <div class="flex items-center gap-md">
        <h1 class="font-headline-md text-headline-md font-bold text-on-surface dark:text-on-surface">{title}</h1>
        {#if subtitle}
            <span class="font-label-sm text-label-sm text-on-surface-variant px-sm py-xs bg-surface-container-low rounded-full">{subtitle}</span>
        {/if}
    </div>
    <div class="flex items-center gap-md">
        <div class="flex items-center gap-sm">
            <ThemeToggle />
            <button class="text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low p-sm rounded-full transition-colors flex items-center justify-center">
                <span class="material-symbols-outlined">calendar_today</span>
            </button>
            <button class="text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low p-sm rounded-full transition-colors flex items-center justify-center">
                <span class="material-symbols-outlined">tune</span>
            </button>
        </div>
        <button class="font-label-md text-label-md text-primary bg-primary/5 hover:bg-primary/10 px-md py-sm rounded-lg transition-colors flex items-center gap-xs">
            <span class="material-symbols-outlined text-[18px]">share</span>
            Share
        </button>
        <button class="font-label-md text-label-md text-on-primary bg-primary hover:bg-surface-tint px-md py-sm rounded-lg shadow-sm transition-all">
            Add Limit
        </button>
    </div>
</header>
