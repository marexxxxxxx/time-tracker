<script lang="ts">
    let { onapply, onclose }: { onapply: (filters: string[]) => void; onclose: () => void } = $props();

    const categories = ['Coding', 'Design', 'Communication', 'Entertainment', 'Neutral'];
    let selected = $state<string[]>([...categories]);

    function toggle(cat: string) {
        if (selected.includes(cat)) {
            selected = selected.filter(c => c !== cat);
        } else {
            selected = [...selected, cat];
        }
    }
</script>

<div class="fixed inset-0 z-50" onclick={onclose} onkeydown={(e) => e.key === 'Escape' && onclose()}>
    <div class="absolute top-[72px] right-[240px] bg-surface-container-high border border-outline-variant/30 rounded-xl shadow-lg p-md w-[220px]" onclick={(e) => e.stopPropagation()}>
        <p class="font-label-md text-on-surface font-semibold mb-sm">Filter by Category</p>
        {#each categories as cat}
            <label class="flex items-center gap-sm py-xs cursor-pointer hover:bg-surface-container-low rounded px-sm">
                <input type="checkbox" checked={selected.includes(cat)} onchange={() => toggle(cat)} class="accent-primary" />
                <span class="font-body-md text-on-surface">{cat}</span>
            </label>
        {/each}
        <div class="flex justify-end gap-sm mt-md">
            <button class="px-md py-sm rounded-lg font-label-md text-on-surface-variant hover:bg-surface-container-low" onclick={onclose}>Cancel</button>
            <button class="px-md py-sm rounded-lg font-label-md text-on-primary bg-primary hover:bg-primary-container" onclick={() => { onapply(selected); onclose(); }}>Apply</button>
        </div>
    </div>
</div>
