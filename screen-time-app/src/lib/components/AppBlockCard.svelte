<script lang="ts">
    let {
        appName,
        icon,
        iconBg,
        limit,
        isBlocked,
        onToggle,
        onRemove,
        onEditLimits,
        dailyLimit = 0,
        weeklyLimit = 0,
        limitEnabled = false,
        usage = "",
        usagePct = -1
    }: {
        appName: string;
        icon: string;
        iconBg: string;
        limit: string;
        isBlocked: boolean;
        onToggle: () => void;
        onRemove?: () => void;
        onEditLimits?: () => void;
        dailyLimit?: number;
        weeklyLimit?: number;
        limitEnabled?: boolean;
        usage?: string;
        usagePct?: number;
    } = $props();

    let limitText = $derived.by(() => {
        if (!limitEnabled) return limit;
        const parts: string[] = [];
        if (dailyLimit > 0) parts.push(`${dailyLimit}m/day`);
        if (weeklyLimit > 0) parts.push(`${weeklyLimit}m/week`);
        return parts.length > 0 ? parts.join(" · ") : limit;
    });
</script>

<div class="flex items-center justify-between p-lg hover:bg-surface-container-lowest dark:hover:bg-surface-container transition-colors {isBlocked ? 'opacity-60' : ''}">
    <div class="flex items-center gap-md">
        <div class="w-10 h-10 rounded-lg flex items-center justify-center" style="background-color: {iconBg}">
            <span class="material-symbols-outlined">{icon}</span>
        </div>
        <div>
            <h4 class="font-body-md text-body-md font-medium text-on-surface">{appName}</h4>
            <p class="font-label-sm text-label-sm text-on-surface-variant">{limitText}</p>
        </div>
    </div>
    <div class="flex items-center gap-md">
        {#if usagePct >= 0}
            <div class="bg-surface-variant dark:bg-surface-container-highest h-1.5 rounded-full w-24 overflow-hidden mr-4">
                <div class="h-1.5 rounded-full" style="width: {usagePct}%; background-color: {iconBg}"></div>
            </div>
            <p class="font-label-sm text-label-sm text-on-surface-variant w-12 text-right mr-4">{usage}</p>
        {/if}
        {#if isBlocked}
            <p class="font-label-sm text-label-sm text-error w-auto mr-4 flex items-center gap-xs">
                <span class="material-symbols-outlined text-[14px]">lock</span>
                Blocked
            </p>
        {/if}
        {#if limitEnabled && !isBlocked}
            <p class="font-label-sm text-label-sm text-tertiary w-auto mr-4 flex items-center gap-xs">
                <span class="material-symbols-outlined text-[14px]">schedule</span>
                Limited
            </p>
        {/if}
        {#if onEditLimits}
            <button
                class="p-xs rounded-lg text-on-surface-variant hover:text-on-surface hover:bg-surface-container-low dark:hover:bg-surface-container transition-colors"
                onclick={() => { alert('TUNE CLICKED: ' + appName); onEditLimits?.(); }}
                aria-label="Edit limits for {appName}"
            >
                <span class="material-symbols-outlined text-[18px]">tune</span>
            </button>
        {/if}
        {#if onRemove}
            <button
                class="p-xs rounded-lg text-on-surface-variant hover:text-error hover:bg-error/10 transition-colors"
                onclick={onRemove}
                aria-label="Remove {appName}"
            >
                <span class="material-symbols-outlined text-[18px]">delete</span>
            </button>
        {/if}
        <button
            class="relative inline-block w-12 align-middle select-none transition duration-200 ease-in"
            onclick={onToggle}
            aria-label="Toggle {appName}"
        >
            <div class="block overflow-hidden h-6 rounded-full transition-colors duration-200 ease-in-out {isBlocked ? 'bg-primary' : 'bg-surface-variant dark:bg-surface-container-highest'}">
                <div class="absolute top-[2px] left-[2px] w-5 h-5 bg-white rounded-full shadow-sm transition-transform duration-200 ease-in-out {isBlocked ? 'translate-x-[24px]' : 'translate-x-0'}"></div>
            </div>
        </button>
    </div>
</div>
