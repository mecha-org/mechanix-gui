<script lang="ts">
	import Layout from '$lib/components/layout.svelte';
	import { goBack } from '$lib/services/common-services';
	import { page } from '$app/stores';
	import {
		ChangePinTypesInfo,
		ChangePinTypes,
		selectedPinLength,
		oldPin
	} from '$lib/stores/securityStore';
	import { authenticate_pin  } from '$lib/services/security-service';
	import PinDialog from '$lib/components/pin-dialog.svelte';
	import { goto } from '$app/navigation';

	// let pinValue: string = '';
	$: pinValue = '';
	const state: any = $page.state;
	const screenType: ChangePinTypes = state?.screenType;

	let showError: boolean = false;
	let errorMessage: string = '';

	const clickHandler = async (event: CustomEvent<any>) => {
		const key = event.detail;
		let updatePin: string = '';
		if (updatePin.length != selectedPinLength) updatePin = pinValue + key;

		switch (key) {
			case 'cancel':
				goto('/security');
				break;

			case 'backspace':
				if (pinValue !== '') pinValue = pinValue.slice(0, -1);
				console.log('updated pinValue: ', pinValue);
				break;

			default:
				if (updatePin.length < selectedPinLength) {
					console.log('UPDATE INPUT', pinValue.length, selectedPinLength);
					pinValue = updatePin;
				} else {
					if (updatePin.length == selectedPinLength) pinValue = updatePin;
					console.log('ELSE API CALL');

					try {
						// const response = await authenticate_pin(pinValue, secret);
						const response = await authenticate_pin(pinValue);
						console.log('authenticate_pin response: ', response);

						if (!response) {
							showError = true;
							errorMessage = 'Pin is incorrect, retry!';
							pinValue = '';
						} else {
							goto('/security/change-pin', {
								invalidateAll: true,
								state: { screenType: ChangePinTypes.SET_PIN }
							});
						}

						setTimeout(() => {
							showError = false;
						}, 2000);
					} catch (error) {
						console.error('AUTHENTICATE_PIN ERROR: ', error);
					}
				}
				break;
		}
	};
</script>

<Layout title="">
	<PinDialog
		bind:title={ChangePinTypesInfo[screenType].title}
		bind:pinValue
		bind:showError
		bind:errorMessage
		on:click={clickHandler}
	></PinDialog>
</Layout>
