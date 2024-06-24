import { fetchingLockStatus, currentLockStatus } from "$lib/stores/securityStore";
import { invoke } from "@tauri-apps/api";

export const get_lock_status = async () => {
    console.log("service::security::get_lock_status() ");
    try {
        // const response: any = await invoke('get_security_lock_status');
        // console.log("get_lock_status response: ", response);

        // true :
        // pin is already set

        // false: 
        // no pin is set

        const response = false;
        currentLockStatus.set(response);
        fetchingLockStatus.set(false);
        return response;
    } catch (error) {
        console.error('service::security::get_lock_status()::error:::: ', error);
        fetchingLockStatus.set(false);
        return error;
    }
};

export const set_pin_lock = async () => {
    console.log("service::security::set_pin_lock() ");

    try {
        // const response: any = await invoke('set_pin_lock');
        const response = true;
        return response;

    } catch (error) {
        console.error('service::security::set_pin_lock()::error:::: ', error);
        return error;

    }
};

export const authenticate_pin = async (pin: string) => {
    console.log("service::security::authenticate_pin() ", pin);

    try {
        // const response: any = await invoke('authenticate_pin', {pin: pin});
        const response = true;
        return response;

    } catch (error) {
        console.error('service::security::authenticate_pin()::error:::: ', error);
        return error;

    }
};


export const set_pin = async (pin: string) => {
    console.log("service::security::set_pin() ", pin);

    try {
        // const response: any = await invoke('set_pin', {pin: pin});
        const response = true;
        return response;

    } catch (error) {
        console.error('service::security::set_pin()::error:::: ', error);
        return error;

    }
};

export const remove_pin_lock = async () => {
    console.log("service::security::remove_pin_lock() ");

    try {
        // const response: any = await invoke('remove_pin_lock');
        const response = true;
        return response;

    } catch (error) {
        console.error('service::security::remove_pin_lock()::error:::: ', error);
        return error;

    }
};