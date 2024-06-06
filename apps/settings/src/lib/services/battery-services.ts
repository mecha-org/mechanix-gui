import { batteryPerformanceMode, batteryPercentage, batteryPerformanceOptions } from "$lib/stores/batteryStore";
import { invoke } from "@tauri-apps/api";

export const get_battery_percentage = async () => {
    console.log("service::battery::get_battery_percentage() ");
    try {
        const response: any = await invoke('get_battery_percentage');
        console.log("get_battery_percentage response: ", response);
        batteryPercentage.set(response);
        return response;
    } catch (error) {
        console.error('service::battery::get_battery_percentage()::error:::: ', error);
        return error;
    }
}

export const get_all_performance_mode = async () => {
    console.log("service::battery::get_all_performance_mode() ");
    try {
        const response: any = await invoke('get_avilable_performance_modes');
        console.log("get_all_performance_mode response: ", response);
        batteryPerformanceOptions.set(response);
        return response;
    } catch (error) {
        console.error('service::battery::get_all_performance_mode()::error:::: ', error);
        return error;
    }
}

export const get_selected_performance_mode = async () => {
    console.log("service::battery::get_selected_performance_mode() ");
    try {
        const response: any = await invoke('get_current_performance_mode');
        console.log("get_selected_performance_mode response: ", response);
        batteryPerformanceMode.set(response);
        return response;
    } catch (error) {
        console.error('service::battery::get_selected_performance_mode()::error:::: ', error);
        return error;
    }
}

export const set_performance_mode = async (value: string) => {
    console.log("service::battery::set_performance_mode() ", value);
    try {
        const response: any = await invoke('set_performance_mode', {value: value});
        console.log("set_performance_mode response: ", response);
        batteryPerformanceMode.set(value);
        return response;
    } catch (error) {
        console.error('service::battery::set_performance_mode()::error:::: ', error);
        return error;
    }
}