<script lang="ts">
    import { theme } from '$lib/stores/theme';
</script>

<button
    class="text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low p-sm rounded-full transition-colors flex items-center justify-center"
    onclick={() => theme.toggle()}
    aria-label="Toggle dark mode"
>
    {#if $theme === 'dark'}
        <span class="material-symbols-outlined">light_mode</span>
    {:else}
        <span class="material-symbols-outlined">dark_mode</span>
    {/if}
</button>
