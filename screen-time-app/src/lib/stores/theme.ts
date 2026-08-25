import { writable } from 'svelte/store';
import { browser } from '$app/environment';

function createThemeStore() {
    const stored = browser ? localStorage.getItem('theme') as 'light' | 'dark' | null : null;
    const initial = stored ?? 'light';
    const { subscribe, set, update } = writable<'light' | 'dark'>(initial);

    if (browser) {
        document.documentElement.classList.toggle('dark', initial === 'dark');
    }

    return {
        subscribe,
        toggle: () => {
            update(current => {
                const next = current === 'light' ? 'dark' : 'light';
                if (browser) {
                    localStorage.setItem('theme', next);
                    document.documentElement.classList.toggle('dark', next === 'dark');
                }
                return next;
            });
        },
        set: (value: 'light' | 'dark') => {
            set(value);
            if (browser) {
                localStorage.setItem('theme', value);
                document.documentElement.classList.toggle('dark', value === 'dark');
            }
        }
    };
}

export const theme = createThemeStore();
