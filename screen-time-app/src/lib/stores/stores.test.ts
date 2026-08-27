import { describe, it, expect, vi, beforeEach } from 'vitest';
import { get } from 'svelte/store';
import type { Activity } from './activities';

vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn(),
}));

import {
    activities,
    totalDuration,
    productivityScore,
    deepWorkSessions,
    formatDuration,
} from './activities';

function setActivities(data: Activity[]) {
    activities.set(data);
}

describe('totalDuration derived store', () => {
    beforeEach(() => {
        activities.set([]);
    });

    it('returns 0 for empty activities', () => {
        expect(get(totalDuration)).toBe(0);
    });

    it('sums durations', () => {
        setActivities([
            { id: 1, app_name: 'A', title: 't', start_time: '', end_time: '', duration: 100, category: 'Coding', productivity_score: 1 },
            { id: 2, app_name: 'B', title: 't', start_time: '', end_time: '', duration: 200, category: 'Neutral', productivity_score: 0 },
        ]);
        expect(get(totalDuration)).toBe(300);
    });

    it('updates when activities change', () => {
        setActivities([
            { id: 1, app_name: 'A', title: 't', start_time: '', end_time: '', duration: 50, category: 'Coding', productivity_score: 1 },
        ]);
        expect(get(totalDuration)).toBe(50);

        setActivities([
            { id: 1, app_name: 'A', title: 't', start_time: '', end_time: '', duration: 50, category: 'Coding', productivity_score: 1 },
            { id: 2, app_name: 'B', title: 't', start_time: '', end_time: '', duration: 150, category: 'Neutral', productivity_score: 0 },
        ]);
        expect(get(totalDuration)).toBe(200);
    });
});

describe('productivityScore derived store', () => {
    beforeEach(() => {
        activities.set([]);
    });

    it('returns 0 for empty activities', () => {
        expect(get(productivityScore)).toBe(0);
    });

    it('calculates score as percentage of productive time', () => {
        setActivities([
            { id: 1, app_name: 'VS Code', title: 't', start_time: '', end_time: '', duration: 300, category: 'Coding', productivity_score: 1 },
            { id: 2, app_name: 'YouTube', title: 't', start_time: '', end_time: '', duration: 100, category: 'Entertainment', productivity_score: -1 },
        ]);
        expect(get(productivityScore)).toBe(75);
    });

    it('returns 100 for all productive', () => {
        setActivities([
            { id: 1, app_name: 'A', title: 't', start_time: '', end_time: '', duration: 100, category: 'Coding', productivity_score: 1 },
            { id: 2, app_name: 'B', title: 't', start_time: '', end_time: '', duration: 200, category: 'Writing', productivity_score: 1 },
        ]);
        expect(get(productivityScore)).toBe(100);
    });

    it('returns 0 for no productive time', () => {
        setActivities([
            { id: 1, app_name: 'YouTube', title: 't', start_time: '', end_time: '', duration: 100, category: 'Entertainment', productivity_score: -1 },
        ]);
        expect(get(productivityScore)).toBe(0);
    });
});

describe('deepWorkSessions derived store', () => {
    beforeEach(() => {
        activities.set([]);
    });

    it('filters for productive sessions >= 30 min', () => {
        const now = new Date().toISOString();
        setActivities([
            { id: 1, app_name: 'VS Code', title: 't1', start_time: now, end_time: now, duration: 1800, category: 'Coding', productivity_score: 1 },
            { id: 2, app_name: 'VS Code', title: 't2', start_time: now, end_time: now, duration: 900, category: 'Coding', productivity_score: 1 },
            { id: 3, app_name: 'YouTube', title: 't3', start_time: now, end_time: now, duration: 3600, category: 'Entertainment', productivity_score: -1 },
            { id: 4, app_name: 'VS Code', title: 't4', start_time: now, end_time: now, duration: 5400, category: 'Coding', productivity_score: 1 },
        ]);
        const sessions = get(deepWorkSessions);
        expect(sessions.length).toBe(2);
        expect(sessions[0].app_name).toBe('VS Code');
    });

    it('returns empty for no qualifying sessions', () => {
        setActivities([
            { id: 1, app_name: 'YouTube', title: 't', start_time: '', end_time: '', duration: 600, category: 'Entertainment', productivity_score: -1 },
        ]);
        expect(get(deepWorkSessions).length).toBe(0);
    });
});
