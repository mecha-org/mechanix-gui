import { writable } from 'svelte/store';

export interface BluetoothData {
    bluetooth_status?: boolean,
    available_devices?: any[],
    paired_devices?: any[]
}
export const bluetoothStore = writable<BluetoothData>({} as BluetoothData);