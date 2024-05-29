import { availableDevicesList, bluetoothStatus, otherDevicesList } from "$lib/stores/bluetoothStore";
import { invoke } from "@tauri-apps/api";


export interface BluetoothScanResponse {
    address: string,
    address_type: string,
    name: string,
    icon: string,
    class: string,
    rssi: string,
    tx_power: string,
    is_paired: boolean,
    is_trusted: boolean,
}
export interface BluetoothScanListResponse {
    bluetooth_devices: BluetoothScanResponse[],
}

export const fetchBluetoothStatus = async () => {
    console.log("service::bluetooth::fetchBluetoothStatus()");
    try {
        const response: boolean = await invoke('get_bluetooth_status');
        console.log("fetchBluetoothStatus response: ", response);
        bluetoothStatus.set(response);
        return response;
    } catch (error) {
        console.error('service::bluetooth::fetchBluetoothStatus()::error:::: ', error);
        return false;
    }
}


export const fetchAvailableDevices = async () => {
    console.log("service::bluetooth::fetchAvailableDevices()");
    try {
        const scan_response: BluetoothScanResponse[]  = (await invoke<BluetoothScanListResponse>('scan_bluetooth')).bluetooth_devices;

        let available_devices: BluetoothScanResponse[] = scan_response?.filter((item: any) => {
            return item.is_paired || item.is_trusted;
        });
        let other_devices: BluetoothScanResponse[] = scan_response.filter((item: any) => {
            return !item.is_paired;
        });

        availableDevicesList.set(available_devices);
        otherDevicesList.set(other_devices);
    } catch (error) {
        console.error('service::bluetooth::fetchAvailableDevices()::error:::: ', error);
        return [] as BluetoothScanResponse[];
    }
}

export const addBluetoothDevice = async (address: string) => {
    console.log("service::bluetooth::addBluetoothDevice()");

    try {
        await invoke('connect_bluetooth_device', { address: address });
    } catch (error) {
        console.error('service::bluetooth::addBluetoothDevice()::error:::: ', error);
        return error;
    }
}

export const forgetBluetoothDevice = async (address: string) => {
    console.log("service::bluetooth::forgetBluetoothDevice()");
    
    try {
        await invoke('disconnect_bluetooth_device', { address });
    } catch (error) {
        console.error('service::bluetooth::forgetBluetoothDevice()::error::::', error);
        return error;
    }
};
