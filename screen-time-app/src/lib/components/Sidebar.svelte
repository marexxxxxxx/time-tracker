<script lang="ts">
    import { page } from '$app/stores';
</script>

<nav class="hidden md:flex bg-surface/80 dark:bg-inverse-surface/80 backdrop-blur-3xl fixed left-0 top-0 h-full w-[280px] border-r border-outline-variant/20 dark:border-outline/20 shadow-sm dark:shadow-none flex-col py-xl px-md gap-lg z-50">
    <!-- Header -->
    <div class="flex items-center gap-md px-md mb-md">
        <div class="w-10 h-10 rounded-full bg-primary-container text-on-primary-container flex items-center justify-center font-bold overflow-hidden border border-outline-variant/30">
            <span class="material-symbols-outlined">person</span>
        </div>
        <div>
            <div class="font-headline-md text-headline-md font-bold text-primary dark:text-primary-fixed-dim">Screen Time</div>
            <div class="font-label-sm text-label-sm text-on-surface-variant">Last synced: 2m ago</div>
        </div>
    </div>

    <!-- Primary Tabs -->
    <div class="flex flex-col gap-sm flex-1">
        <a class="{$page.url.pathname === '/' ? 'bg-primary/10 dark:bg-primary-container/20 text-primary dark:text-primary-fixed-dim font-bold' : 'text-on-surface-variant dark:text-surface-variant hover:bg-surface-container-low dark:hover:bg-surface-container-highest/20'} rounded-lg px-md py-sm flex items-center gap-sm active:scale-[0.98] transition-all duration-150" href="/">
            <span class="material-symbols-outlined" style={$page.url.pathname === '/' ? "font-variation-settings: 'FILL' 1;" : ""}>dashboard</span>
            <span class="font-label-md text-label-md">Overview</span>
        </a>
        <a class="{$page.url.pathname.includes('/blocked') ? 'bg-primary/10 dark:bg-primary-container/20 text-primary dark:text-primary-fixed-dim font-bold' : 'text-on-surface-variant dark:text-surface-variant hover:bg-surface-container-low dark:hover:bg-surface-container-highest/20'} rounded-lg px-md py-sm flex items-center gap-sm transition-colors duration-200" href="/blocked">
            <span class="material-symbols-outlined" style={$page.url.pathname.includes('/blocked') ? "font-variation-settings: 'FILL' 1;" : ""}>block</span>
            <span class="font-label-md text-label-md">Blocked Apps</span>
        </a>
        <a class="{$page.url.pathname.includes('/productivity') ? 'bg-primary/10 dark:bg-primary-container/20 text-primary dark:text-primary-fixed-dim font-bold' : 'text-on-surface-variant dark:text-surface-variant hover:bg-surface-container-low dark:hover:bg-surface-container-highest/20'} rounded-lg px-md py-sm flex items-center gap-sm transition-colors duration-200" href="/productivity">
            <span class="material-symbols-outlined" style={$page.url.pathname.includes('/productivity') ? "font-variation-settings: 'FILL' 1;" : ""}>bar_chart</span>
            <span class="font-label-md text-label-md">Productivity Tracker</span>
        </a>
    </div>

    <!-- Footer Tabs -->
    <div class="flex flex-col gap-sm mt-auto pt-md border-t border-outline-variant/20">
        <a class="text-on-surface-variant dark:text-surface-variant hover:bg-surface-container-low dark:hover:bg-surface-container-highest/20 rounded-lg px-md py-sm flex items-center gap-sm transition-colors duration-200" href="#">
            <span class="material-symbols-outlined">settings</span>
            <span class="font-label-md text-label-md">Settings</span>
        </a>
        <a class="text-on-surface-variant dark:text-surface-variant hover:bg-surface-container-low dark:hover:bg-surface-container-highest/20 rounded-lg px-md py-sm flex items-center gap-sm transition-colors duration-200" href="#">
            <span class="material-symbols-outlined">help</span>
            <span class="font-label-md text-label-md">Help</span>
        </a>
    </div>
</nav>
