import type { KnownNetworkResponse, WirelessInfoResponse } from '$lib/services/network-services';
import { writable } from 'svelte/store';

// network page
export const wifiStatus = writable(false);
export const connectedNetwork = writable<WirelessInfoResponse>({} as WirelessInfoResponse);
export const disableWifiSwitch = writable(false);
export const fetchingWifiStatus = writable(true);

// manage-network page


export const fetchingAvailableNetworks = writable(true);

export const fetchingKnownNetworks = writable(true);

export const knownNetworksList = writable<KnownNetworkResponse[]>([] as KnownNetworkResponse[]);
export const availableNetworksList = writable<WirelessInfoResponse[]>([] as WirelessInfoResponse[]);
