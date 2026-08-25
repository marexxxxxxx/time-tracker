import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface BlockedApp {
    id: number;
    app_name: string;
    is_blocked: boolean;
}

export const blockedApps = writable<BlockedApp[]>([]);

export async function fetchBlockedApps() {
    try {
        const data: BlockedApp[] = await invoke('get_blocked_apps');
        blockedApps.set(data);
    } catch (e) {
        console.error("Failed to fetch blocked apps:", e);
    }
}

export async function addBlockedApp(appName: string) {
    try {
        await invoke('add_blocked_app', { appName });
        await fetchBlockedApps();
    } catch (e) {
        console.error("Failed to add blocked app:", e);
    }
}

export async function removeBlockedApp(id: number) {
    try {
        await invoke('remove_blocked_app', { id });
        await fetchBlockedApps();
    } catch (e) {
        console.error("Failed to remove blocked app:", e);
    }
}

export async function toggleBlockedApp(id: number) {
    try {
        await invoke('toggle_blocked_app', { id });
        await fetchBlockedApps();
    } catch (e) {
        console.error("Failed to toggle blocked app:", e);
    }
}
