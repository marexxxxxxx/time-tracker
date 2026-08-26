<script lang="ts">
    let { onselect, onclose }: { onselect: (date: string) => void; onclose: () => void } = $props();
    let selectedDate = $state(new Date().toISOString().split('T')[0]);

    function handleSelect() {
        onselect(selectedDate);
        onclose();
    }
</script>

<div class="fixed inset-0 z-50" onclick={onclose} onkeydown={(e) => e.key === 'Escape' && onclose()}>
    <div class="absolute top-[72px] right-[200px] bg-surface-container-high border border-outline-variant/30 rounded-xl shadow-lg p-md w-[280px]" onclick={(e) => e.stopPropagation()}>
        <input
            type="date"
            bind:value={selectedDate}
            class="w-full bg-surface-container-low border border-outline-variant/30 rounded-lg px-md py-sm font-body-md text-on-surface focus:border-primary focus:ring-2 focus:ring-primary/20 outline-none"
        />
        <div class="flex justify-end gap-sm mt-md">
            <button class="px-md py-sm rounded-lg font-label-md text-on-surface-variant hover:bg-surface-container-low" onclick={onclose}>Cancel</button>
            <button class="px-md py-sm rounded-lg font-label-md text-on-primary bg-primary hover:bg-primary-container" onclick={handleSelect}>Go</button>
        </div>
    </div>
</div>
