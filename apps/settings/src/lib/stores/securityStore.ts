import { writable } from "svelte/store";

export const currentLockStatus = writable<boolean>(false);

export const fetchingLockStatus = writable<boolean>(true);
export const disableLockSwitch = writable(false);
