import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Activity {
    id: number;
    app_name: string;
    title: string;
    start_time: string;
    end_time: string;
    duration: number;
    category: string;
    productivity_score: number;
}

export const activities = writable<Activity[]>([]);
export const isLoading = writable(true);

export async function fetchActivities() {
    isLoading.set(true);
    try {
        const data: Activity[] = await invoke('get_activities');
        activities.set(data);
    } catch (e) {
        console.error("Failed to fetch activities:", e);
    } finally {
        isLoading.set(false);
    }
}

// Derived store to calculate total duration in seconds
export const totalDuration = derived(activities, $activities => {
    return $activities.reduce((sum, act) => sum + act.duration, 0);
});

// Derived store to calculate productivity score percentage
export const productivityScore = derived(activities, $activities => {
    if ($activities.length === 0) return 0;
    const totalDuration = $activities.reduce((sum, act) => sum + act.duration, 0);
    const productiveDuration = $activities
        .filter(act => act.productivity_score > 0)
        .reduce((sum, act) => sum + act.duration, 0);

    return Math.round((productiveDuration / totalDuration) * 100);
});

// Derived store for deep work sessions (filtering for productive work >= 30 mins)
export const deepWorkSessions = derived(activities, $activities => {
    return $activities
        .filter(act => act.productivity_score > 0 && act.duration >= 1800)
        .sort((a, b) => new Date(b.start_time).getTime() - new Date(a.start_time).getTime());
});

// Format duration helper
export function formatDuration(seconds: number): string {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    if (h > 0 && m > 0) return `${h}h ${m}m`;
    if (h > 0) return `${h}h`;
    return `${m}m`;
}