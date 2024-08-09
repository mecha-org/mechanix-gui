import { writable } from "svelte/store";

export enum DeviceType {
    INPUT='INPUT',
    OUTPUT='OUTPUT'
}

export interface SoundDevice {
    name: string,
    description: string,
    prop_list: any,
    sound_level?: [number],
    is_mute?: boolean
}


export const inputDevices = writable<SoundDevice[]>([] as SoundDevice[]);
export const outputDevices = writable<SoundDevice[]>([] as SoundDevice[]);
