import { writable } from 'svelte/store';

export const isIdle = writable(false);

export interface IdleStatePayload {
    is_idle: boolean;
    idle_time_seconds: number;
}

export function parseIdleEvent(payload: string): IdleStatePayload | null {
    try {
        const parsed = JSON.parse(payload);
        if (typeof parsed.is_idle !== 'boolean') return null;
        return {
            is_idle: parsed.is_idle,
            idle_time_seconds: typeof parsed.idle_time_seconds === 'number' ? parsed.idle_time_seconds : 0,
        };
    } catch (e) {
        return null;
    }
}