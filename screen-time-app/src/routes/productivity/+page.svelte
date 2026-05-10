<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
</script>

<TopBar title="Productivity Tracker" />

<main class="flex-1 overflow-y-auto p-margin-desktop mt-[88px]">
    <div class="max-w-[1200px] mx-auto grid grid-cols-12 gap-lg pb-xxl">
        <!-- Summary Card -->
        <div class="col-span-12 md:col-span-4 bg-surface-container-lowest border border-outline-variant rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)] flex flex-col justify-between">
            <div>
                <h3 class="font-label-md text-label-md text-on-surface-variant mb-xs uppercase tracking-wider">Productivity Score</h3>
                <div class="flex items-end gap-sm mb-md">
                    <span class="font-display text-display text-primary">84%</span>
                    <span class="font-label-sm text-label-sm text-secondary bg-secondary-container/30 px-xs py-[2px] rounded mb-sm flex items-center"><span class="material-symbols-outlined text-[12px] mr-1">arrow_upward</span> +5%</span>
                </div>
            </div>
            <div class="space-y-sm">
                <div class="flex justify-between items-center font-label-md text-label-md">
                    <span class="text-on-surface-variant">Focus Time</span>
                    <span class="text-on-surface font-semibold">5h 20m</span>
                </div>
                <div class="w-full bg-surface-variant rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 75%"></div>
                </div>
            </div>
        </div>

        <!-- Work vs Leisure Chart -->
        <div class="col-span-12 md:col-span-8 bg-surface-container-lowest border border-outline-variant rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)]">
            <div class="flex justify-between items-center mb-xl">
                <h3 class="font-headline-md text-headline-md text-on-surface font-semibold">Work vs. Leisure</h3>
                <div class="flex gap-md font-label-sm text-label-sm">
                    <div class="flex items-center gap-xs"><div class="w-3 h-3 rounded-full bg-primary"></div> Productive</div>
                    <div class="flex items-center gap-xs"><div class="w-3 h-3 rounded-full bg-surface-variant"></div> Neutral</div>
                    <div class="flex items-center gap-xs"><div class="w-3 h-3 rounded-full bg-tertiary"></div> Leisure</div>
                </div>
            </div>
            <!-- Simulated Bar Chart -->
            <div class="flex items-end gap-sm h-48 mt-md border-b border-outline-variant/30 pb-sm">
                <!-- Day 1 -->
                <div class="flex-1 flex flex-col justify-end group">
                    <div class="w-full bg-tertiary rounded-t-full h-[20%] transition-all opacity-80 group-hover:opacity-100"></div>
                    <div class="w-full bg-surface-variant h-[10%] transition-all opacity-80 group-hover:opacity-100"></div>
                    <div class="w-full bg-primary h-[60%] transition-all opacity-80 group-hover:opacity-100 rounded-b-sm"></div>
                    <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm">Mon</div>
                </div>
                <!-- Day 2 -->
                <div class="flex-1 flex flex-col justify-end group">
                    <div class="w-full bg-tertiary rounded-t-full h-[15%] transition-all opacity-80 group-hover:opacity-100"></div>
                    <div class="w-full bg-surface-variant h-[5%] transition-all opacity-80 group-hover:opacity-100"></div>
                    <div class="w-full bg-primary h-[70%] transition-all opacity-80 group-hover:opacity-100 rounded-b-sm"></div>
                    <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm">Tue</div>
                </div>
                <!-- Day 3 -->
                <div class="flex-1 flex flex-col justify-end group">
                    <div class="w-full bg-tertiary rounded-t-full h-[30%] transition-all opacity-80 group-hover:opacity-100"></div>
                    <div class="w-full bg-surface-variant h-[20%] transition-all opacity-80 group-hover:opacity-100"></div>
                    <div class="w-full bg-primary h-[40%] transition-all opacity-80 group-hover:opacity-100 rounded-b-sm"></div>
                    <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm">Wed</div>
                </div>
                <!-- Day 4 (Today) -->
                <div class="flex-1 flex flex-col justify-end group relative">
                    <div class="absolute -top-6 left-1/2 -translate-x-1/2 bg-surface-variant text-on-surface font-label-sm text-[10px] px-2 py-1 rounded-md opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap">8.2 hrs</div>
                    <div class="w-full bg-tertiary rounded-t-full h-[10%] transition-all opacity-80 group-hover:opacity-100"></div>
                    <div class="w-full bg-surface-variant h-[10%] transition-all opacity-80 group-hover:opacity-100"></div>
                    <div class="w-full bg-primary h-[80%] transition-all opacity-80 group-hover:opacity-100 rounded-b-sm"></div>
                    <div class="text-center font-label-sm text-label-sm text-primary font-bold mt-sm">Thu</div>
                </div>
                <!-- Day 5 -->
                <div class="flex-1 flex flex-col justify-end group">
                    <div class="w-full bg-surface-variant/30 rounded-t-full h-[5%] border border-dashed border-outline-variant/50"></div>
                    <div class="text-center font-label-sm text-label-sm text-on-surface-variant mt-sm">Fri</div>
                </div>
            </div>
        </div>

        <!-- Category Breakdown -->
        <div class="col-span-12 md:col-span-5 bg-surface-container-lowest border border-outline-variant rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)]">
            <h3 class="font-headline-md text-headline-md text-on-surface font-semibold mb-lg">Categories</h3>
            <div class="space-y-md">
                <!-- Category Item -->
                <div>
                    <div class="flex justify-between items-center mb-xs">
                        <div class="flex items-center gap-sm">
                            <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
                                <span class="material-symbols-outlined text-[18px]">code</span>
                            </div>
                            <span class="font-body-md text-body-md font-medium">Coding</span>
                        </div>
                        <span class="font-label-md text-label-md text-on-surface-variant">3h 45m</span>
                    </div>
                    <div class="w-full bg-surface-variant rounded-full h-1.5 ml-10 w-[calc(100%-40px)]">
                        <div class="bg-primary h-1.5 rounded-full" style="width: 65%"></div>
                    </div>
                </div>

                <!-- Category Item -->
                <div>
                    <div class="flex justify-between items-center mb-xs">
                        <div class="flex items-center gap-sm">
                            <div class="w-8 h-8 rounded-lg bg-tertiary/10 flex items-center justify-center text-tertiary">
                                <span class="material-symbols-outlined text-[18px]">draw</span>
                            </div>
                            <span class="font-body-md text-body-md font-medium">Design</span>
                        </div>
                        <span class="font-label-md text-label-md text-on-surface-variant">1h 20m</span>
                    </div>
                    <div class="w-full bg-surface-variant rounded-full h-1.5 ml-10 w-[calc(100%-40px)]">
                        <div class="bg-tertiary h-1.5 rounded-full" style="width: 25%"></div>
                    </div>
                </div>

                <!-- Category Item -->
                <div>
                    <div class="flex justify-between items-center mb-xs">
                        <div class="flex items-center gap-sm">
                            <div class="w-8 h-8 rounded-lg bg-secondary/10 flex items-center justify-center text-secondary">
                                <span class="material-symbols-outlined text-[18px]">edit_document</span>
                            </div>
                            <span class="font-body-md text-body-md font-medium">Writing</span>
                        </div>
                        <span class="font-label-md text-label-md text-on-surface-variant">45m</span>
                    </div>
                    <div class="w-full bg-surface-variant rounded-full h-1.5 ml-10 w-[calc(100%-40px)]">
                        <div class="bg-secondary h-1.5 rounded-full" style="width: 10%"></div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Deep Work Sessions -->
        <div class="col-span-12 md:col-span-7 bg-surface-container-lowest border border-outline-variant rounded-xl p-lg shadow-[0_4px_20px_rgba(0,0,0,0.04)]">
            <div class="flex justify-between items-center mb-lg">
                <h3 class="font-headline-md text-headline-md text-on-surface font-semibold">Recent Deep Work</h3>
                <button class="text-primary font-label-sm text-label-sm hover:underline">View All</button>
            </div>
            <div class="space-y-sm">
                <!-- Session -->
                <div class="flex items-center justify-between p-sm hover:bg-surface-container-low rounded-lg transition-colors group cursor-pointer border border-transparent hover:border-outline-variant/30">
                    <div class="flex items-center gap-md">
                        <div class="w-2 h-10 bg-primary rounded-full"></div>
                        <div>
                            <p class="font-body-md text-body-md font-medium text-on-surface">VS Code Architecture</p>
                            <p class="font-label-sm text-label-sm text-on-surface-variant flex items-center gap-xs">
                                <span class="material-symbols-outlined text-[14px]">schedule</span> 09:00 AM - 11:30 AM
                            </p>
                        </div>
                    </div>
                    <div class="text-right">
                        <p class="font-headline-md text-body-md font-semibold text-primary">2h 30m</p>
                        <span class="inline-block px-2 py-0.5 rounded-md bg-surface-variant text-on-surface-variant font-label-sm text-[10px] uppercase">Coding</span>
                    </div>
                </div>

                <!-- Session -->
                <div class="flex items-center justify-between p-sm hover:bg-surface-container-low rounded-lg transition-colors group cursor-pointer border border-transparent hover:border-outline-variant/30">
                    <div class="flex items-center gap-md">
                        <div class="w-2 h-10 bg-tertiary rounded-full"></div>
                        <div>
                            <p class="font-body-md text-body-md font-medium text-on-surface">Figma UI Kit Update</p>
                            <p class="font-label-sm text-label-sm text-on-surface-variant flex items-center gap-xs">
                                <span class="material-symbols-outlined text-[14px]">schedule</span> 01:00 PM - 02:20 PM
                            </p>
                        </div>
                    </div>
                    <div class="text-right">
                        <p class="font-headline-md text-body-md font-semibold text-tertiary">1h 20m</p>
                        <span class="inline-block px-2 py-0.5 rounded-md bg-surface-variant text-on-surface-variant font-label-sm text-[10px] uppercase">Design</span>
                    </div>
                </div>

                <!-- Session -->
                <div class="flex items-center justify-between p-sm hover:bg-surface-container-low rounded-lg transition-colors group cursor-pointer border border-transparent hover:border-outline-variant/30">
                    <div class="flex items-center gap-md">
                        <div class="w-2 h-10 bg-primary rounded-full"></div>
                        <div>
                            <p class="font-body-md text-body-md font-medium text-on-surface">Code Review &amp; PRs</p>
                            <p class="font-label-sm text-label-sm text-on-surface-variant flex items-center gap-xs">
                                <span class="material-symbols-outlined text-[14px]">schedule</span> 03:00 PM - 04:15 PM
                            </p>
                        </div>
                    </div>
                    <div class="text-right">
                        <p class="font-headline-md text-body-md font-semibold text-primary">1h 15m</p>
                        <span class="inline-block px-2 py-0.5 rounded-md bg-surface-variant text-on-surface-variant font-label-sm text-[10px] uppercase">Coding</span>
                    </div>
                </div>
            </div>
        </div>
    </div>
</main>
