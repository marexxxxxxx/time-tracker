import { describe, it, expect } from 'vitest';
import { formatDuration } from './activities';

describe('formatDuration', () => {
    it('formats 0 seconds as 0m', () => {
        expect(formatDuration(0)).toBe('0m');
    });

    it('formats seconds less than a minute as 0m', () => {
        expect(formatDuration(45)).toBe('0m');
    });

    it('formats exactly 60 seconds as 1m', () => {
        expect(formatDuration(60)).toBe('1m');
    });

    it('formats minutes only', () => {
        expect(formatDuration(900)).toBe('15m');
        expect(formatDuration(3540)).toBe('59m');
    });

    it('formats exactly 1 hour', () => {
        expect(formatDuration(3600)).toBe('1h');
    });

    it('formats hours and minutes', () => {
        expect(formatDuration(3660)).toBe('1h 1m');
        expect(formatDuration(5400)).toBe('1h 30m');
        expect(formatDuration(7200)).toBe('2h');
    });

    it('formats large durations', () => {
        expect(formatDuration(36000)).toBe('10h');
        expect(formatDuration(39600)).toBe('11h');
    });
});
