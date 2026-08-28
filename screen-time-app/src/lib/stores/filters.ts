import { writable } from 'svelte/store';

export const allCategories = ['Coding', 'Design', 'Communication', 'Entertainment', 'Neutral'];
export const activeCategories = writable<string[]>([...allCategories]);
