import { fetchingLockStatus, currentLockStatus, oldPin } from "$lib/stores/securityStore";
import { invoke } from "@tauri-apps/api";

export const get_lock_status = async () => {
    console.log("service::security::get_lock_status() ");
    try {
        const response: any = await invoke('get_security_lock_status');
        console.log("get_lock_status response: ", response);
        currentLockStatus.set(response);
        fetchingLockStatus.set(false);
        return response;
    } catch (error) {
        console.error('service::security::get_lock_status()::error:::: ', error);
        fetchingLockStatus.set(false);
        return error;
    }
};

export const authenticate_pin = async (pin: string) => {
    console.log("service::security::authenticate_pin() ", pin);

    try {
        const response: any = await invoke('authenticate_pin', { pin: pin });
        console.log("service::security::authenticate_pin()::response: ", pin);
        oldPin.set(pin);
        // TODO: 
        // to handle enable or disable lock switch 
        return response;
    } catch (error) {
        console.error('service::security::authenticate_pin()::error:::: ', error);
        return error;

    }
};

export const set_pin_lock = async (oldPin: string, newPin: string, setNewSecret: boolean) => {
    console.log("service::security::set_pin_lock() ", { oldPin, newPin });

    try {
        const response: any = await invoke('change_pin', { oldPin: oldPin, newPin: newPin, setNewSecret: setNewSecret });
        console.log("service::security::set_pin_lock():response:: ", response);
        get_lock_status();  // to handle enable or disable lock switch
        return response;

    } catch (error) {
        console.error('service::security::set_pin_lock()::error:::: ', error);
        return error;

    }
};

// TODO RUST API
export const remove_pin_lock = async (pin: string) => {
    console.log("service::security::remove_pin_lock() ");

    try {
        const response: any = await invoke('remove_pin_lock', {pin: pin});
        return response;
    } catch (error) {
        console.error('service::security::remove_pin_lock()::error:::: ', error);
        return error;

    }
};