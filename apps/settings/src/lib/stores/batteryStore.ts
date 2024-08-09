import { writable } from 'svelte/store';

const defaultOptions : string[]= ['Low', 'Balanced', 'High'];
export const batteryPerformanceOptions = writable<string[]>(defaultOptions);
export const batteryPerformanceMode = writable<string>('');
export const batteryPercentage = writable<number>(0);