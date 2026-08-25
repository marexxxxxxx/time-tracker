<script lang="ts">
    let { title, description = "" }: { title: string; description?: string } = $props();
</script>

<div class="mb-lg">
    <h2 class="font-headline-lg text-headline-lg text-on-surface mb-xs font-semibold">{title}</h2>
    {#if description}
        <p class="font-body-md text-body-md text-on-surface-variant">{description}</p>
    {/if}
</div>
