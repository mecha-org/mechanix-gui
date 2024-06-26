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

// export const set_pin_secret = async () => {
//     console.log("service::security::set_pin_secret() ");

//     try {
//         const response: any = await invoke('set_pin_secret');
//         console.log("service::security::set_pin_secret() response===>  ", response);
//         pinSecret.set(response);
//         return response;
//     } catch (error) {
//         console.error('service::security::set_pin_secret()::error:::: ', error);
//         return error;

//     }
// };

// export const get_pin_secret = async () => {
//     console.log("service::security::get_pin_secret() ");

//     try {
//         const response: any = await invoke('get_pin_secret');
//         return response;
//     } catch (error) {
//         console.error('service::security::get_pin_secret()::error:::: ', error);
//         return error;
//     }
// };

export const authenticate_pin = async (pin: string) => {
    console.log("service::security::authenticate_pin() ", pin);

    try {
        const response: any = await invoke('authenticate_pin', { pin: pin });
        console.log("service::security::authenticate_pin()::response: ", pin);
        oldPin.set(pin);
        return response;

    } catch (error) {
        console.error('service::security::authenticate_pin()::error:::: ', error);
        return error;

    }
};

export const set_pin_lock = async (oldPin: string, newPin: string, setPinEnabled: boolean) => {
    console.log("service::security::set_pin_lock() ", { oldPin, newPin });

    try {
        const response: any = await invoke('change_pin', { oldPin: oldPin, newPin: newPin, setPinEnabled: setPinEnabled });
        console.log("service::security::set_pin_lock():response:: ", response);
        return response;

    } catch (error) {
        console.error('service::security::set_pin_lock()::error:::: ', error);
        return error;

    }
};

// TODO RUST API
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