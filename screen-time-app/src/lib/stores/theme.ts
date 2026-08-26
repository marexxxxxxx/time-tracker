import { writable } from 'svelte/store';
import { browser } from '$app/environment';

type Theme = 'system' | 'light' | 'dark';

function getSystemTheme(): 'light' | 'dark' {
    if (!browser) return 'light';
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function applyTheme(theme: Theme) {
    if (!browser) return;
    const effective = theme === 'system' ? getSystemTheme() : theme;
    document.documentElement.classList.toggle('dark', effective === 'dark');
}

function createThemeStore() {
    const stored = browser ? localStorage.getItem('theme') as Theme | null : null;
    const initial = stored ?? 'system';
    const { subscribe, set, update } = writable<Theme>(initial);

    if (browser) {
        applyTheme(initial);

        // Listen for system theme changes when in 'system' mode
        const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
        mediaQuery.addEventListener('change', () => {
            const current = localStorage.getItem('theme') as Theme;
            if (current === 'system' || !current) {
                applyTheme('system');
            }
        });
    }

    return {
        subscribe,
        toggle: () => {
            update(current => {
                const cycle: Theme[] = ['system', 'light', 'dark'];
                const nextIndex = (cycle.indexOf(current) + 1) % cycle.length;
                const next = cycle[nextIndex];
                if (browser) {
                    localStorage.setItem('theme', next);
                    applyTheme(next);
                }
                return next;
            });
        },
        set: (value: Theme) => {
            set(value);
            if (browser) {
                localStorage.setItem('theme', value);
                applyTheme(value);
            }
        }
    };
}

export const theme = createThemeStore();
