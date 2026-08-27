<script lang="ts">
    import { theme } from '$lib/stores/theme';

    let currentTheme = $derived($theme);

    function cycleTheme() {
        theme.toggle();
    }

    let icon = $derived(currentTheme === 'system' ? 'brightness_auto' : currentTheme === 'light' ? 'light_mode' : 'dark_mode');
    let label = $derived(currentTheme === 'system' ? 'System' : currentTheme === 'light' ? 'Light' : 'Dark');
</script>

<button
    class="text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low p-sm rounded-full transition-colors flex items-center justify-center"
    onclick={cycleTheme}
    title="Theme: {label}"
>
    <span class="material-symbols-outlined">{icon}</span>
</button>
