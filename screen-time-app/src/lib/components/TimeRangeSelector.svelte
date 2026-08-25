<script lang="ts">
    let { selected = "Day", options = ["Day", "Week", "Month"], onselect }: {
        selected?: string;
        options?: string[];
        onselect?: (opt: string) => void;
    } = $props();

    function select(opt: string) {
        selected = opt;
        onselect?.(opt);
    }
</script>

<div class="flex bg-surface-container-low dark:bg-surface-container-highest p-xs rounded-lg">
    {#each options as opt}
        <button
            class="px-md py-xs font-label-md text-label-md rounded-md transition-colors {selected === opt ? 'bg-surface-container-lowest shadow-sm text-on-surface' : 'text-on-surface-variant hover:text-on-surface'}"
            onclick={() => select(opt)}
        >
            {opt}
        </button>
    {/each}
</div>
