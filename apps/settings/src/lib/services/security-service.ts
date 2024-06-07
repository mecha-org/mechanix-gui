import { fetchingLockStatus, currentLockStatus } from "$lib/stores/securityStore";
import { invoke } from "@tauri-apps/api";

export const get_lock_status = async () => {
    console.log("service::battery::get_lock_status() ");
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
}
