<script lang="ts">
    export let title: string = "Screen Time";
    export let subtitle: string = "";
</script>

<header class="fixed top-0 right-0 left-[280px] z-40 bg-surface/60 dark:bg-inverse-surface/60 backdrop-blur-2xl border-b border-outline-variant/20 dark:border-outline/20 shadow-none flex items-center justify-between px-margin-desktop py-lg w-[calc(100%-280px)] h-[80px]">
    <div class="flex items-center gap-md">
        <h1 class="font-headline-md text-headline-md font-bold text-on-surface dark:text-on-surface">{title}</h1>
        {#if subtitle}
            <span class="font-label-sm text-label-sm text-on-surface-variant px-sm py-xs bg-surface-container-low rounded-full">{subtitle}</span>
        {/if}
    </div>
    <div class="flex items-center gap-md">
        <button class="text-on-surface-variant hover:text-on-surface hover:opacity-80 transition-opacity p-sm rounded-full flex items-center justify-center">
            <span class="material-symbols-outlined" data-icon="calendar_today">calendar_today</span>
        </button>
        <button class="text-on-surface-variant hover:text-on-surface hover:opacity-80 transition-opacity p-sm rounded-full flex items-center justify-center">
            <span class="material-symbols-outlined" data-icon="tune">tune</span>
        </button>
        <button class="text-primary hover:opacity-80 transition-opacity font-label-md text-label-md font-semibold px-md py-sm rounded-full bg-primary/10 flex items-center gap-xs ml-sm">
            <span class="material-symbols-outlined text-[18px]" data-icon="share">share</span>
            Share
        </button>
        <button class="bg-primary text-on-primary hover:opacity-80 transition-opacity font-label-md text-label-md font-semibold px-lg py-sm rounded-full shadow-sm ml-sm">
            Add Limit
        </button>
        <div class="w-8 h-8 rounded-full bg-surface-variant ml-sm overflow-hidden border border-outline-variant/30">
            <!-- Avatar Placeholder -->
        </div>
    </div>
</header>