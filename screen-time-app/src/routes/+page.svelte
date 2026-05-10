<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
</script>

<TopBar title="Overview" subtitle="Today, Oct 24" />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-7xl mx-auto w-full">
    <!-- Bento Grid Layout -->
    <div class="grid grid-cols-12 gap-lg">
        <!-- Summary Card -->
        <div class="col-span-12 lg:col-span-4 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] flex flex-col justify-between">
            <div>
                <div class="flex items-center gap-sm mb-sm">
                    <span class="material-symbols-outlined text-primary">schedule</span>
                    <h2 class="font-label-md text-label-md text-on-surface-variant">Total Screen Time</h2>
                </div>
                <div class="font-display text-display text-on-surface mb-xs">6h 45m</div>
                <div class="font-body-md text-body-md text-secondary flex items-center gap-xs">
                    <span class="material-symbols-outlined text-[16px]">arrow_downward</span>
                    12% from yesterday
                </div>
            </div>
            <div class="mt-xl">
                <div class="flex justify-between font-label-sm text-label-sm text-on-surface-variant mb-sm">
                    <span>Productivity Score</span>
                    <span class="text-on-surface font-semibold">78%</span>
                </div>
                <div class="h-2 w-full bg-surface-container-high rounded-full overflow-hidden">
                    <div class="h-full bg-primary rounded-full w-[78%]"></div>
                </div>
            </div>
        </div>

        <!-- Main Chart Card -->
        <div class="col-span-12 lg:col-span-8 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] flex flex-col">
            <div class="flex justify-between items-center mb-xl">
                <h2 class="font-headline-md text-headline-md text-on-surface">Daily Usage</h2>
                <div class="flex bg-surface-container-low p-xs rounded-lg">
                    <button class="px-md py-xs font-label-md text-label-md bg-surface-container-lowest shadow-sm rounded-md text-on-surface">Day</button>
                    <button class="px-md py-xs font-label-md text-label-md text-on-surface-variant hover:text-on-surface">Week</button>
                </div>
            </div>
            <!-- Faux Bar Chart -->
            <div class="flex-1 flex items-end gap-md pt-lg min-h-[200px]">
                <!-- Bars -->
                <div class="flex-1 flex flex-col justify-end group">
                    <div class="w-full bg-primary/20 hover:bg-primary/40 rounded-t-full h-[30%] transition-colors relative"></div>
                    <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm">6 AM</div>
                </div>
                <div class="flex-1 flex flex-col justify-end group">
                    <div class="w-full bg-primary/20 hover:bg-primary/40 rounded-t-full h-[50%] transition-colors relative"></div>
                    <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm">9 AM</div>
                </div>
                <div class="flex-1 flex flex-col justify-end group">
                    <div class="w-full bg-primary hover:bg-surface-tint rounded-t-full h-[90%] shadow-sm transition-colors relative">
                        <div class="absolute -top-10 left-1/2 -translate-x-1/2 bg-inverse-surface text-inverse-on-surface px-sm py-xs rounded font-label-sm text-label-sm opacity-0 group-hover:opacity-100 transition-opacity">1.5h</div>
                    </div>
                    <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm font-semibold">12 PM</div>
                </div>
                <div class="flex-1 flex flex-col justify-end group">
                    <div class="w-full bg-primary/20 hover:bg-primary/40 rounded-t-full h-[70%] transition-colors relative"></div>
                    <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm">3 PM</div>
                </div>
                <div class="flex-1 flex flex-col justify-end group">
                    <div class="w-full bg-primary/20 hover:bg-primary/40 rounded-t-full h-[40%] transition-colors relative"></div>
                    <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm">6 PM</div>
                </div>
                <div class="flex-1 flex flex-col justify-end group">
                    <div class="w-full bg-primary/20 hover:bg-primary/40 rounded-t-full h-[20%] transition-colors relative"></div>
                    <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm">9 PM</div>
                </div>
            </div>
        </div>

        <!-- Most Used Apps -->
        <div class="col-span-12 lg:col-span-6 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)]">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-xl">Most Used</h2>
            <div class="flex flex-col gap-lg">
                <!-- App Item -->
                <div>
                    <div class="flex justify-between items-center mb-sm">
                        <div class="flex items-center gap-sm">
                            <div class="w-8 h-8 rounded-lg bg-[#000000] flex items-center justify-center">
                                <span class="material-symbols-outlined text-white text-[18px]">code</span>
                            </div>
                            <span class="font-body-md text-body-md text-on-surface">VS Code</span>
                        </div>
                        <span class="font-label-md text-label-md text-on-surface-variant">2h 15m</span>
                    </div>
                    <div class="h-2 w-full bg-surface-container-high rounded-full overflow-hidden">
                        <div class="h-full bg-primary rounded-full w-[45%]"></div>
                    </div>
                </div>

                <!-- App Item -->
                <div>
                    <div class="flex justify-between items-center mb-sm">
                        <div class="flex items-center gap-sm">
                            <div class="w-8 h-8 rounded-lg bg-[#0070eb] flex items-center justify-center">
                                <span class="material-symbols-outlined text-white text-[18px]">explore</span>
                            </div>
                            <span class="font-body-md text-body-md text-on-surface">Safari</span>
                        </div>
                        <span class="font-label-md text-label-md text-on-surface-variant">1h 45m</span>
                    </div>
                    <div class="h-2 w-full bg-surface-container-high rounded-full overflow-hidden">
                        <div class="h-full bg-[#0070eb] rounded-full w-[35%] opacity-80"></div>
                    </div>
                </div>

                <!-- App Item -->
                <div>
                    <div class="flex justify-between items-center mb-sm">
                        <div class="flex items-center gap-sm">
                            <div class="w-8 h-8 rounded-lg bg-[#4A154B] flex items-center justify-center">
                                <span class="material-symbols-outlined text-white text-[18px]">forum</span>
                            </div>
                            <span class="font-body-md text-body-md text-on-surface">Slack</span>
                        </div>
                        <span class="font-label-md text-label-md text-on-surface-variant">55m</span>
                    </div>
                    <div class="h-2 w-full bg-surface-container-high rounded-full overflow-hidden">
                        <div class="h-full bg-tertiary rounded-full w-[15%]"></div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Categories -->
        <div class="col-span-12 lg:col-span-6 bg-surface-container-lowest border border-outline-variant/30 rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)]">
            <h2 class="font-headline-md text-headline-md text-on-surface mb-xl">Categories</h2>
            <div class="flex gap-lg items-center h-[calc(100%-48px)]">
                <!-- Faux Donut Chart -->
                <div class="relative w-32 h-32 flex-shrink-0">
                    <svg class="w-full h-full transform -rotate-90" viewBox="0 0 36 36">
                        <path class="text-surface-container-high" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" stroke-width="4"></path>
                        <path class="text-primary" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" stroke-dasharray="60, 100" stroke-width="4"></path>
                        <path class="text-secondary" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" stroke-dasharray="25, 100" stroke-dashoffset="-60" stroke-width="4"></path>
                        <path class="text-tertiary" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" stroke-dasharray="15, 100" stroke-dashoffset="-85" stroke-width="4"></path>
                    </svg>
                </div>
                <!-- Legend -->
                <div class="flex-1 flex flex-col gap-md">
                    <div class="flex justify-between items-center">
                        <div class="flex items-center gap-sm">
                            <div class="w-3 h-3 rounded-full bg-primary"></div>
                            <span class="font-body-md text-body-md text-on-surface">Productivity</span>
                        </div>
                        <span class="font-label-md text-label-md text-on-surface-variant">60%</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <div class="flex items-center gap-sm">
                            <div class="w-3 h-3 rounded-full bg-secondary"></div>
                            <span class="font-body-md text-body-md text-on-surface">Communication</span>
                        </div>
                        <span class="font-label-md text-label-md text-on-surface-variant">25%</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <div class="flex items-center gap-sm">
                            <div class="w-3 h-3 rounded-full bg-tertiary"></div>
                            <span class="font-body-md text-body-md text-on-surface">Entertainment</span>
                        </div>
                        <span class="font-label-md text-label-md text-on-surface-variant">15%</span>
                    </div>
                </div>
            </div>
        </div>
    </div>
</main>
