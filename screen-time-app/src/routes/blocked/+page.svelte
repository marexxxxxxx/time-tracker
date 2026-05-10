<script lang="ts">
    import TopBar from '$lib/components/TopBar.svelte';
</script>

<TopBar title="Screen Time" />

<main class="flex-1 px-margin-desktop py-xxl mt-[88px] max-w-7xl mx-auto w-full">
    <div class="max-w-4xl mx-auto space-y-xl">
        <!-- Page Header -->
        <div class="mb-lg">
            <h2 class="font-headline-lg text-headline-lg text-on-surface mb-xs font-semibold">Blocked Apps &amp; Limits</h2>
            <p class="font-body-md text-body-md text-on-surface-variant">Manage restrictions and focus schedules for your applications.</p>
        </div>

        <!-- Social Media Section -->
        <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
            <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Social Media</h3>
                <span class="font-label-sm text-label-sm text-on-surface-variant bg-surface-variant/50 px-sm py-xs rounded-md">3 Apps</span>
            </div>
            <div class="divide-y divide-outline-variant/20">
                <!-- App Item -->
                <div class="flex items-center justify-between p-lg hover:bg-surface-container-lowest transition-colors">
                    <div class="flex items-center gap-md">
                        <div class="w-10 h-10 rounded-lg bg-[#E1306C]/10 text-[#E1306C] flex items-center justify-center">
                            <span class="material-symbols-outlined" data-icon="photo_camera">photo_camera</span>
                        </div>
                        <div>
                            <h4 class="font-body-md text-body-md font-medium text-on-surface">Instagram</h4>
                            <p class="font-label-sm text-label-sm text-on-surface-variant">Limit: 30m / day</p>
                        </div>
                    </div>
                    <div class="flex items-center gap-md">
                        <div class="w-full bg-surface-variant h-1.5 rounded-full w-24 overflow-hidden mr-4">
                            <div class="bg-[#E1306C] h-1.5 rounded-full" style="width: 80%;"></div>
                        </div>
                        <p class="font-label-sm text-label-sm text-on-surface-variant w-12 text-right mr-4">24m used</p>
                        <!-- Toggle -->
                        <div class="relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in">
                            <input checked class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer z-10 transition-transform duration-200 ease-in-out border-primary top-0" id="toggle1" name="toggle" style="right: 0;" type="checkbox"/>
                            <label class="toggle-label block overflow-hidden h-6 rounded-full bg-primary cursor-pointer transition-colors duration-200 ease-in-out" for="toggle1"></label>
                        </div>
                    </div>
                </div>

                <!-- App Item -->
                <div class="flex items-center justify-between p-lg hover:bg-surface-container-lowest transition-colors">
                    <div class="flex items-center gap-md">
                        <div class="w-10 h-10 rounded-lg bg-black/5 text-black flex items-center justify-center">
                            <span class="material-symbols-outlined" data-icon="music_note">music_note</span>
                        </div>
                        <div>
                            <h4 class="font-body-md text-body-md font-medium text-on-surface">TikTok</h4>
                            <p class="font-label-sm text-label-sm text-on-surface-variant">Blocked entirely</p>
                        </div>
                    </div>
                    <div class="flex items-center gap-md">
                        <p class="font-label-sm text-label-sm text-error w-auto mr-4 flex items-center gap-xs">
                            <span class="material-symbols-outlined text-[14px]">lock</span>
                            Blocked
                        </p>
                        <!-- Toggle -->
                        <div class="relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in">
                            <input checked class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer z-10 transition-transform duration-200 ease-in-out border-primary top-0" id="toggle2" name="toggle" style="right: 0;" type="checkbox"/>
                            <label class="toggle-label block overflow-hidden h-6 rounded-full bg-primary cursor-pointer transition-colors duration-200 ease-in-out" for="toggle2"></label>
                        </div>
                    </div>
                </div>

                <!-- App Item -->
                <div class="flex items-center justify-between p-lg hover:bg-surface-container-lowest transition-colors opacity-60">
                    <div class="flex items-center gap-md">
                        <div class="w-10 h-10 rounded-lg bg-[#1DA1F2]/10 text-[#1DA1F2] flex items-center justify-center">
                            <span class="material-symbols-outlined" data-icon="chat_bubble">chat_bubble</span>
                        </div>
                        <div>
                            <h4 class="font-body-md text-body-md font-medium text-on-surface">Twitter</h4>
                            <p class="font-label-sm text-label-sm text-on-surface-variant">Limit: 1h / day</p>
                        </div>
                    </div>
                    <div class="flex items-center gap-md">
                        <div class="w-full bg-surface-variant h-1.5 rounded-full w-24 overflow-hidden mr-4">
                            <div class="bg-[#1DA1F2] h-1.5 rounded-full" style="width: 10%;"></div>
                        </div>
                        <p class="font-label-sm text-label-sm text-on-surface-variant w-12 text-right mr-4">6m used</p>
                        <!-- Toggle -->
                        <div class="relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in">
                            <input class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer z-10 transition-transform duration-200 ease-in-out border-surface-variant top-0" id="toggle3" name="toggle" style="left: 0;" type="checkbox"/>
                            <label class="toggle-label block overflow-hidden h-6 rounded-full bg-surface-variant cursor-pointer transition-colors duration-200 ease-in-out" for="toggle3"></label>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <!-- Entertainment Section -->
        <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
            <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Entertainment</h3>
                <span class="font-label-sm text-label-sm text-on-surface-variant bg-surface-variant/50 px-sm py-xs rounded-md">2 Apps</span>
            </div>
            <div class="divide-y divide-outline-variant/20">
                <!-- App Item -->
                <div class="flex items-center justify-between p-lg hover:bg-surface-container-lowest transition-colors">
                    <div class="flex items-center gap-md">
                        <div class="w-10 h-10 rounded-lg bg-[#FF0000]/10 text-[#FF0000] flex items-center justify-center">
                            <span class="material-symbols-outlined" data-icon="play_arrow">play_arrow</span>
                        </div>
                        <div>
                            <h4 class="font-body-md text-body-md font-medium text-on-surface">YouTube</h4>
                            <p class="font-label-sm text-label-sm text-on-surface-variant">Limit: 2h / day (Weekends Only)</p>
                        </div>
                    </div>
                    <div class="flex items-center gap-md">
                        <!-- Toggle -->
                        <div class="relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in">
                            <input checked class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer z-10 transition-transform duration-200 ease-in-out border-primary top-0" id="toggle4" name="toggle" style="right: 0;" type="checkbox"/>
                            <label class="toggle-label block overflow-hidden h-6 rounded-full bg-primary cursor-pointer transition-colors duration-200 ease-in-out" for="toggle4"></label>
                        </div>
                    </div>
                </div>

                <!-- App Item -->
                <div class="flex items-center justify-between p-lg hover:bg-surface-container-lowest transition-colors">
                    <div class="flex items-center gap-md">
                        <div class="w-10 h-10 rounded-lg bg-[#E50914]/10 text-[#E50914] flex items-center justify-center">
                            <span class="material-symbols-outlined" data-icon="movie">movie</span>
                        </div>
                        <div>
                            <h4 class="font-body-md text-body-md font-medium text-on-surface">Netflix</h4>
                            <p class="font-label-sm text-label-sm text-on-surface-variant">Limit: 1h 30m / day</p>
                        </div>
                    </div>
                    <div class="flex items-center gap-md">
                        <!-- Toggle -->
                        <div class="relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in">
                            <input checked class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer z-10 transition-transform duration-200 ease-in-out border-primary top-0" id="toggle5" name="toggle" style="right: 0;" type="checkbox"/>
                            <label class="toggle-label block overflow-hidden h-6 rounded-full bg-primary cursor-pointer transition-colors duration-200 ease-in-out" for="toggle5"></label>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <!-- Custom Limits Section -->
        <section class="bg-surface rounded-xl border border-outline-variant/40 glass-shadow overflow-hidden">
            <div class="px-lg py-md border-b border-outline-variant/20 bg-surface-container-low/50 flex justify-between items-center">
                <h3 class="font-label-md text-label-md font-semibold text-on-surface uppercase tracking-wider">Custom Limits</h3>
                <button class="text-primary hover:bg-primary/5 transition-colors font-label-md text-label-md font-semibold px-sm py-xs rounded-md flex items-center gap-xs">
                    <span class="material-symbols-outlined text-[16px]" data-icon="add">add</span>
                    New Limit
                </button>
            </div>
            <div class="p-xl flex flex-col items-center justify-center text-center gap-sm">
                <div class="w-16 h-16 rounded-full bg-surface-variant/50 flex items-center justify-center text-outline mb-sm">
                    <span class="material-symbols-outlined text-[32px]" data-icon="dashboard_customize">dashboard_customize</span>
                </div>
                <p class="font-body-md text-body-md text-on-surface-variant max-w-md">You haven't set up any custom app or website group limits yet. Group related apps to share a single time pool.</p>
                <button class="mt-sm bg-surface-variant/50 hover:bg-surface-variant text-on-surface transition-colors font-label-md text-label-md font-semibold px-lg py-md rounded-full border border-outline-variant/30">
                    Create Custom Limit
                </button>
            </div>
        </section>
    </div>
</main>
