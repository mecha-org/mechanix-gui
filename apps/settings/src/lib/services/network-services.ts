import { availableNetworksList, connectedNetwork, knownNetworksList, wifiStatus } from "$lib/stores/networkStore";
import type { KnownNetworkListResponse, WirelessScanListResponse, WirelessInfoResponse } from "$lib/types/NetworkTypes";
import { invoke } from "@tauri-apps/api/tauri";


export const fetchKnownNetworks = () => {
    console.log("service::network::fetchKnownNetworks()");
    return invoke<KnownNetworkListResponse>('get_known_networks')
        .then((response) => {
            if (response) {
                knownNetworksList.set((response as KnownNetworkListResponse).known_network);
            }
            return response;
        })
        .catch((error: Error) => {
            console.error('service::network::fetchKnownNetworks()::error:::: ', error);
            return {} as KnownNetworkListResponse;
        });

}


export const fetchAvaialbleNetworks = () => {
    console.log("service::network::fetchAvaialbleNetworks()");
    return invoke<WirelessScanListResponse>('wifi_scanning')
        .then((response) => {
            if (response) {
                // // NOTE: Keep only if issue occurs of duplicate networks
                // // required filter out - flags like TEMP
                // const filteredResponse = (response as WirelessScanListResponse).wireless_network
                //     .filter((x: any) => !x.flags.includes("TEMP"));
                // const finalResponse = {wireless_network: filteredResponse};

                availableNetworksList.set((response as WirelessScanListResponse).wireless_network);
            }
            return response;
        })
        .catch((error: Error) => {
            console.error('service::network::fetchAvaialbleNetworks()::error:::: ', error);
            return {} as WirelessScanListResponse;
        });
}



export const fetchWifiStatus = async () => {
    console.log("service::network::fetchWifiStatus()");
    try {
        const response: boolean = await invoke('get_wifi_status');
        wifiStatus.set(response);
        return response;
    } catch (error) {
        console.error('service::network::fetchWifiStatus()::error:::: ', error);
        return false;
    }

}


export const fetchConnectedWifiInfo = async () => {
    console.log("service::network::fetchConnectedWifiInfo()");
    try {
        const wifi_info_response: WirelessInfoResponse = await invoke('get_connected_wifi_info');
        connectedNetwork.set(wifi_info_response); 
        return wifi_info_response;
    } catch (error) {
        console.error('service::network::fetchConnectedWifiInfo()::error:::: ', error);
        return {} as WirelessInfoResponse;
    }

}

export const removeWifi = async (networkSsid: string) => {
    try {
        const response: boolean = await invoke('disconnect_network', { networkSsid: networkSsid });
        console.log("service::network::removeWifi()::response :: ", response);
        return response;
    } catch (error) {
        console.error('service::network::removeWifi()::error:::: ', error);
        return error;
    }
}