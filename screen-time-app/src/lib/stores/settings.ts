import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { browser } from '$app/environment';

interface Settings {
    theme: 'system' | 'light' | 'dark';
    idle_timeout: string;
    tracking_paused: string;
    limit_warnings: string;
    daily_summary: string;
}

const defaults: Settings = {
    theme: 'system',
    idle_timeout: '5',
    tracking_paused: 'false',
    limit_warnings: 'true',
    daily_summary: 'true',
};

function createSettingsStore() {
    const { subscribe, set, update } = writable<Settings>({ ...defaults });
    let loaded = false;

    return {
        subscribe,
        load: async () => {
            if (!browser || loaded) return;
            try {
                const response = await invoke<{ settings: Record<string, string> }>('get_settings');
                const merged = { ...defaults, ...response.settings };
                set(merged);
                loaded = true;
            } catch (e) {
                console.error('Failed to load settings:', e);
            }
        },
        update: async (key: keyof Settings, value: string) => {
            update(s => ({ ...s, [key]: value }));
            try {
                await invoke('update_setting', { key, value });
            } catch (e) {
                console.error('Failed to save setting:', e);
            }
        },
        reset: () => {
            set({ ...defaults });
            loaded = false;
        }
    };
}

export const settings = createSettingsStore();
