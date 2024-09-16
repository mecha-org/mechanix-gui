import { inputDevices, outputDevices, type SoundDevice  } from "$lib/stores/soundStore";
import { invoke } from "@tauri-apps/api";

export const getInputDevices = async () => {
    console.log("service::sound::getInputDevices()");
    try {
        const response: SoundDevice[] = await invoke('get_input_devices');
        console.log("getInputDevices response: ", response);
        inputDevices.set(response);
        return response;
      
        //  const filter = response.filter(device => 
        //     !device.name.includes("output")
        // );
        // console.log("FILTERED INPUT: ", filter);
        // inputDevices.set(filter);
        // return filter;
    } catch (error) {
        console.error('service::sound::getInputDevices()::error:::: ', error);
        return error;
    }
}

export const getOutputDevices = async () => {
    console.log("service::sound::getOutputDevices()");
    try {
        const response: SoundDevice[] = await invoke('get_output_devices');
        console.log("getOutputDevices response: ", response);
        outputDevices.set(response);
        return response;

        // const filter = response.filter(device => 
        //     device.name.includes("input")
        // );
        // console.log("FILTERED OUTPUT: ", filter);
        // outputDevices.set(filter);
        // return filter;
    } catch (error) {
        console.error('service::sound::getOutputDevices()::error:::: ', error);
        return error;
    }
}

export const getInputDeviceVolume = async (name: string) => {
    console.log("service::sound::getInputDeviceVolume()", name);
    try 
    {
        // const response: any= await invoke('get_input_sound_value', {device: name});
        // temp = passing "" to get default value
        const response: any= await invoke('get_input_sound_value', {device: ""});
        console.log("getInputDeviceVolume response: ", response);
        return response;
    } catch (error) {
        console.error('service::sound::getInputDeviceVolume()::error:::: ', error);
        return error;
    }
}

export const setInputDeviceVolume = async (value: number, device: string) => {
    console.log("service::sound::setInputDeviceVolume()", device);
    try 
    {
        // const response: any= await invoke('set_input_sound_value', {value: value, device: device});
        // temp
        const response: any= await invoke('set_input_sound_value', {value: value, device: ""});   
        console.log("setInputDeviceVolume response: ", response);
        return response;
    } catch (error) {
        console.error('service::sound::setInputDeviceVolume()::error:::: ', error);
        return error;
    }
}

export const getOutputDeviceVolume = async (name: string) => {
    console.log("service::sound::getOutputDeviceVolume()", name);
    try 
    {
        // const response: any= await invoke('get_output_sound_value', {device: name});
        const response: any= await invoke('get_output_sound_value', {device: ""});
        console.log("getOutputDeviceVolume response: ", response);
        return response;
    } catch (error) {
        console.error('service::sound::getOutputDeviceVolume()::error:::: ', error);
        return error;
    }
}

export const setOutputDeviceVolume = async (value: number, device: string) => {
    console.log("service::sound::setOutputDeviceVolume()", device);
    try 
    {
        // const response: any= await invoke('set_output_sound_value', {value: value, device: device});
        const response: any= await invoke('set_output_sound_value', {value: value, device: ""});
        console.log("setOutputDeviceVolume response: ", response);
        return response;
    } catch (error) {
        console.error('service::sound::setOutputDeviceVolume()::error:::: ', error);
        return error;
    }
} 


export const getAllInputDevicesVolume = async (devices: SoundDevice[]) => {
    console.log("service::sound::getAllInputDevicesVolume()", devices);
    try 
    {
        const updatedDevices: SoundDevice[]  = await Promise.all(
            devices.map(async (device) => {
              const volume = await getInputDeviceVolume(device.name);
            //   console.log("name :: ", device.name, "VOLUME: ", volume);
              return { ...device, sound_level: [volume], is_mute: volume==0 ? true: false };
            })
          );
         inputDevices.set(updatedDevices);
        return updatedDevices;
    } catch (error) {
        console.error('service::sound::getAllInputDevicesVolume()::error:::: ', error);
        return error;
    }
}

export const getAllOutputDevicesVolume = async (devices: SoundDevice[]) => {
    console.log("service::sound::getAllOutputDevicesVolume()", devices);
    try 
    {
        const updatedDevices: SoundDevice[]  = await Promise.all(
            devices.map(async (device) => {
              const volume = await getOutputDeviceVolume(device.name);
            //   console.log("name :: ", device.name, "VOLUME: ", volume);
              return { ...device, sound_level: [volume], is_mute: volume==0 ? true: false  };
            })
          );
        outputDevices.set(updatedDevices);
        return updatedDevices;
    } catch (error) {
        console.error('service::sound::getAllOutputDevicesVolume()::error:::: ', error);
        return error;
    }
}

export const updateInputDeviceMute = async(value: string) => {
    console.log("service::sound::updateInputDeviceMute()", value);
    try {
        const result = await invoke('input_device_toggle_mute', {device: value});
        console.log("TOGGLE MUTE result: ", result);
    } catch (error) {
        console.error('service::sound::updateInputDeviceMute()::error:::: ', error);
    }
}

export const updateOutputDeviceMute = async(value: string) => {
    console.log("service::sound::updateOutputDeviceMute()", value);
    try {
        await invoke('output_device_toggle_mute', {device: value});
    } catch (error) {
        console.error('service::sound::updateOutputDeviceMute()::error:::: ', error);
    }
}

