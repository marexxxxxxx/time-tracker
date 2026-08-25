import { writable } from 'svelte/store';

export type TimeRange = 'Day' | 'Week' | 'Month';
export const selectedRange = writable<TimeRange>('Day');
