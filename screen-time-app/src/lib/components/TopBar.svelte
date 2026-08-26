<script lang="ts">
    import ThemeToggle from './ThemeToggle.svelte';
    import FilterPopover from './FilterPopover.svelte';
    import QuickAddLimitModal from './QuickAddLimitModal.svelte';

    let { title = "Screen Time", subtitle = "" }: { title?: string; subtitle?: string } = $props();

    let showFilter = $state(false);
    let showAddLimit = $state(false);

    function handleFilterApply(filters: string[]) {
        console.log('Active filters:', filters);
    }
</script>

<header data-tauri-drag-region class="fixed top-0 right-0 left-[280px] z-40 bg-surface/60 backdrop-blur-2xl border-b border-outline-variant/20 shadow-none flex items-center justify-between px-margin-desktop py-lg w-[calc(100%-280px)] h-[88px]">
    <div class="flex items-center gap-md">
        <h1 class="font-headline-md text-headline-md font-bold text-on-surface">{title}</h1>
        {#if subtitle}
            <span class="font-label-sm text-label-sm text-on-surface-variant px-sm py-xs bg-surface-container-low rounded-full">{subtitle}</span>
        {/if}
    </div>
    <div class="flex items-center gap-md">
        <div class="flex items-center gap-sm">
            <ThemeToggle />
            <button class="text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low p-sm rounded-full transition-colors flex items-center justify-center" onclick={() => showFilter = !showFilter}>
                <span class="material-symbols-outlined">tune</span>
            </button>
        </div>
        <button class="font-label-md text-label-md text-on-primary bg-primary hover:bg-surface-tint px-md py-sm rounded-lg shadow-sm transition-all" onclick={() => showAddLimit = true}>
            Add Limit
        </button>
    </div>
</header>

{#if showFilter}
    <FilterPopover onapply={handleFilterApply} onclose={() => showFilter = false} />
{/if}

<QuickAddLimitModal open={showAddLimit} onclose={() => showAddLimit = false} />
