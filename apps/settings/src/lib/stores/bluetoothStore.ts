import { writable } from 'svelte/store';

import type { BluetoothScanResponse } from '$lib/services/bluetooth-services';

// bluetooth page
export const bluetoothStatus = writable(false);
export const fetchingBluetoothStatus = writable(true);

export const isFetchingAvailableDevices = writable(true);
export const isFetchingOtherDevices = writable(true);

export const availableDevicesList = writable<BluetoothScanResponse[]>([] as BluetoothScanResponse[]);
export const otherDevicesList = writable<BluetoothScanResponse[]>([] as BluetoothScanResponse[]);
