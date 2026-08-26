<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';

    let { onclose }: { onclose: () => void } = $props();

    async function exportCSV() {
        try {
            const data = await invoke<string>('export_activities_csv');
            const blob = new Blob([data], { type: 'text/csv' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `screentime-export-${new Date().toISOString().split('T')[0]}.csv`;
            a.click();
            URL.revokeObjectURL(url);
        } catch (e) {
            console.error('Export failed:', e);
        }
        onclose();
    }

    async function exportJSON() {
        try {
            const data = await invoke<string>('export_activities_json');
            const blob = new Blob([data], { type: 'application/json' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `screentime-export-${new Date().toISOString().split('T')[0]}.json`;
            a.click();
            URL.revokeObjectURL(url);
        } catch (e) {
            console.error('Export failed:', e);
        }
        onclose();
    }
</script>

<div class="fixed inset-0 z-50" onclick={onclose} onkeydown={(e) => e.key === 'Escape' && onclose()}>
    <div class="absolute top-[72px] right-[160px] bg-surface-container-high border border-outline-variant/30 rounded-xl shadow-lg p-sm w-[180px]" onclick={(e) => e.stopPropagation()}>
        <button class="w-full text-left px-md py-sm rounded-lg font-body-md text-on-surface hover:bg-surface-container-low flex items-center gap-sm" onclick={exportCSV}>
            <span class="material-symbols-outlined text-[18px]">description</span>
            Export CSV
        </button>
        <button class="w-full text-left px-md py-sm rounded-lg font-body-md text-on-surface hover:bg-surface-container-low flex items-center gap-sm" onclick={exportJSON}>
            <span class="material-symbols-outlined text-[18px]">data_object</span>
            Export JSON
        </button>
    </div>
</div>
