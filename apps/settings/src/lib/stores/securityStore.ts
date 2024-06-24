import { writable } from "svelte/store";

export enum ChangePinTypes {
    SET_PIN = "SET_PIN",
    AUTHENTICATE_PIN = "AUTHENTICATE_PIN",
};

export const ChangePinTypesInfo = {
    SET_PIN: {
        title: "Set your pin",
    },
    AUTHENTICATE_PIN: {
        title: "Authenticate your pin",
    }
};

export const pin_lentgh_options = {
    FOUR: 4,
    SIX: 6,
};

export const currentLockStatus = writable<boolean>(false);
export const fetchingLockStatus = writable<boolean>(true);

export const disableLockSwitch = writable<boolean>(false);
export const selectedPinLength = pin_lentgh_options.FOUR;

