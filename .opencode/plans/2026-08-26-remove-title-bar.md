# Remove Native Title Bar

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove the native OS title bar from the Tauri window so the app has a clean, borderless look with the TopBar as the only header.

**Architecture:** Disable Tauri's native window decorations and add a drag region to the TopBar so the window remains movable.

**Tech Stack:** Tauri v2, Svelte 5

**Spec:** This plan — no separate spec needed for a 2-file change.

## Global Constraints

- Tauri v2 (`"tauri" ^2`)
- Linux target (X11 + Wayland)
- No window controls (close via Alt+F4 or taskbar)

---

### Task 1: Disable native decorations + add drag region

**Files:**
- Modify: `screen-time-app/src-tauri/tauri.conf.json:12-18`
- Modify: `screen-time-app/src/lib/components/TopBar.svelte:24`

- [ ] **Step 1: Add `decorations: false` to Tauri window config**

In `src-tauri/tauri.conf.json`, change the window object from:

```json
{
  "title": "screen-time-app",
  "width": 800,
  "height": 600
}
```

to:

```json
{
  "title": "screen-time-app",
  "width": 800,
  "height": 600,
  "decorations": false
}
```

- [ ] **Step 2: Add drag region to TopBar**

In `src/lib/components/TopBar.svelte`, add `data-tauri-drag-region` to the `<header>` element:

```svelte
<header data-tauri-drag-region class="fixed top-0 right-0 left-[280px] z-40 ...">
```

This makes the TopBar draggable so the window can be moved.

- [ ] **Step 3: Verify build**

Run: `cd screen-time-app && npm run build`
Expected: Build succeeds

Run: `cd screen-time-app/src-tauri && cargo check`
Expected: Compiles with only pre-existing warnings

- [ ] **Step 4: Commit**

```bash
git add screen-time-app/src-tauri/tauri.conf.json screen-time-app/src/lib/components/TopBar.svelte
git commit -m "feat: remove native title bar, use TopBar as drag region"
```
