# Tasks 2-5: Screen Time Dashboard Components

**Date:** 2026-08-25
**Status:** DONE

## Summary

Created 5 Svelte 5 components (Tasks 2-5) for the Screen Time Dashboard. All use Svelte 5 runes (`$props()`, `$derived`), M3 color tokens, and Tailwind CSS utility classes.

## Components Created

| Task | Component | File | Purpose |
|------|-----------|------|---------|
| 2 | `StatCard` | `src/lib/components/StatCard.svelte` | Metric card with icon, value, optional progress bar |
| 3 | `AppUsageList` | `src/lib/components/AppUsageList.svelte` | App usage list with colored bars and duration formatting |
| 4 | `CategoryDonut` | `src/lib/components/CategoryDonut.svelte` | SVG donut chart placeholder for categories |
| 5 | `PageHeader` | `src/lib/components/PageHeader.svelte` | Page title and description |
| 5 | `TimeRangeSelector` | `src/lib/components/TimeRangeSelector.svelte` | Day/Week/Month toggle |

## Commits

- `5728f4a` feat: add StatCard component
- `1ab34e8` feat: add AppUsageList component
- `ecf5c7d` feat: add CategoryDonut placeholder component
- `69f5ad2` feat: add PageHeader and TimeRangeSelector components

## Verification

`npm run check` passes with 0 errors (2 pre-existing a11y warnings in Sidebar.svelte).

## Issues Fixed

- **CategoryDonut brief bug:** The provided code referenced `acc[acc.length - 1].pct` but the property is named `percentage`. Fixed to `acc[acc.length - 1].percentage` to match the TypeScript type.

## Notes

- `AppUsageList` imports `formatDuration` from `$lib/stores/activities` (existing store) — verified the import resolves correctly via svelte-check.
- `CategoryDonut` is a placeholder SVG donut; to be replaced with Chart.js in Phase 4 per brief.

## Fix: TimeRangeSelector missing `onselect` callback

**Commit:** `f1fb793` fix: add onselect callback prop to TimeRangeSelector

**What changed:**
- Added `onselect?: (opt: string) => void` to the `$props()` type definition
- `select()` now calls `onselect?.(opt)` after updating local state, allowing parent components to react to range changes

**Verification:** `npm run check` passes with 0 errors (2 pre-existing a11y warnings).
