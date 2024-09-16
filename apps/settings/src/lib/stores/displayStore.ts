import { writable } from "svelte/store";

export const brightnessPercentage = writable<number[]>([0]);
