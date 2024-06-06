import { brightnessPercentage } from "$lib/stores/displayStore";
import { invoke } from "@tauri-apps/api";

export const getBrightness = async () => {
    console.log("service::display::getBrightness()");
    try {
        const response: any = await invoke('get_brightness');
        brightnessPercentage.set(response);
        console.log("getBrightness response: ", response);
        return response;
    } catch (error) {
        console.error('service::display::getBrightness()::error:::: ', error);
        return error;
    }
}

export const setBrightness = async (value: number[]) => {
    console.log("service::display::setBrightness() ", value);
    brightnessPercentage.set(value);   // temp
    try {
        const response: any = await invoke('set_brightness', { value: value[0] });
        console.log("getBrightness response: ", response);
        brightnessPercentage.set(value);
        return response;
    } catch (error) {
        console.error('service::display::setBrightness()::error:::: ', error);
        return error;
    }
} 