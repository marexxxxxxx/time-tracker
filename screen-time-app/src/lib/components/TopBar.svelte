<script lang="ts">
    import ThemeToggle from './ThemeToggle.svelte';
    import DatePickerPopover from './DatePickerPopover.svelte';
    import FilterPopover from './FilterPopover.svelte';
    import ExportDropdown from './ExportDropdown.svelte';
    import QuickAddLimitModal from './QuickAddLimitModal.svelte';

    let { title = "Screen Time", subtitle = "" }: { title?: string; subtitle?: string } = $props();

    let showCalendar = $state(false);
    let showFilter = $state(false);
    let showExport = $state(false);
    let showAddLimit = $state(false);

    function handleDateSelect(date: string) {
        console.log('Selected date:', date);
        // TODO: integrate with timeRange store
    }

    function handleFilterApply(filters: string[]) {
        console.log('Active filters:', filters);
        // TODO: integrate with filter store
    }
</script>

<header class="fixed top-0 right-0 left-[280px] z-40 bg-surface/60 backdrop-blur-2xl border-b border-outline-variant/20 shadow-none flex items-center justify-between px-margin-desktop py-lg w-[calc(100%-280px)] h-[88px]">
    <div class="flex items-center gap-md">
        <h1 class="font-headline-md text-headline-md font-bold text-on-surface">{title}</h1>
        {#if subtitle}
            <span class="font-label-sm text-label-sm text-on-surface-variant px-sm py-xs bg-surface-container-low rounded-full">{subtitle}</span>
        {/if}
    </div>
    <div class="flex items-center gap-md">
        <div class="flex items-center gap-sm">
            <ThemeToggle />
            <button class="text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low p-sm rounded-full transition-colors flex items-center justify-center" onclick={() => showCalendar = !showCalendar}>
                <span class="material-symbols-outlined">calendar_today</span>
            </button>
            <button class="text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low p-sm rounded-full transition-colors flex items-center justify-center" onclick={() => showFilter = !showFilter}>
                <span class="material-symbols-outlined">tune</span>
            </button>
        </div>
        <div class="relative">
            <button class="font-label-md text-label-md text-primary bg-primary/5 hover:bg-primary/10 px-md py-sm rounded-lg transition-colors flex items-center gap-xs" onclick={() => showExport = !showExport}>
                <span class="material-symbols-outlined text-[18px]">share</span>
                Share
            </button>
            {#if showExport}
                <ExportDropdown onclose={() => showExport = false} />
            {/if}
        </div>
        <button class="font-label-md text-label-md text-on-primary bg-primary hover:bg-surface-tint px-md py-sm rounded-lg shadow-sm transition-all" onclick={() => showAddLimit = true}>
            Add Limit
        </button>
    </div>
</header>

{#if showCalendar}
    <DatePickerPopover onselect={handleDateSelect} onclose={() => showCalendar = false} />
{/if}

{#if showFilter}
    <FilterPopover onapply={handleFilterApply} onclose={() => showFilter = false} />
{/if}

<QuickAddLimitModal open={showAddLimit} onclose={() => showAddLimit = false} />
