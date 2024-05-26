import { writable } from 'svelte/store';

export interface BluetoothData {
    bluetooth_status: boolean,
    available_devices: any[],
    other_devices: any[]
}
export const bluetooth_store = writable<BluetoothData>({} as BluetoothData);
