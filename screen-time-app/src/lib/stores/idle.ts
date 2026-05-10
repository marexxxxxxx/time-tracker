import { listen } from '@tauri-apps/api/event';
import { writable } from 'svelte/store';

export interface IdleEvent {
    is_idle: boolean;
    idle_time_seconds: number;
}

export const isIdle = writable(false);

export async function setupIdleListener() {
    console.log("Setting up idle listener");
    const unlisten = await listen<IdleEvent>('idle-state-changed', (event) => {
        console.log("Idle state changed:", event.payload);
        isIdle.set(event.payload.is_idle);

        // When the user becomes idle, we could pause the timer or show a notification.
        // For now, we'll just log and maybe show a banner if needed.
        if (event.payload.is_idle) {
            console.warn(`User is idle for ${event.payload.idle_time_seconds} seconds. Timer paused (mock).`);
        } else {
            console.log("User resumed activity.");
        }
    });

    return unlisten;
}