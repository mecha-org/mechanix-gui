<script lang="ts">
	import Layout from '$lib/components/layout.svelte';
	import { goBack } from '$lib/services/common-services';
	import { page } from '$app/stores';
	import {
		ChangePinTypesInfo,
		ChangePinTypes,
		selectedPinLength,
		oldPin,

	} from '$lib/stores/securityStore';
	import { authenticate_pin, set_pin_lock } from '$lib/services/security-service';
	import PinDialog from '$lib/components/pin-dialog.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	// let pinValue: string = '';
	$: pinValue = '';
	const state: any = $page.state;
	const screenType: ChangePinTypes = state?.screenType;
	const {setPinEnabled} = state;

	let showError: boolean = false;
	let errorMessage: string = '';

	const clickHandler = (event: CustomEvent<any>) => {
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
						
						const response = set_pin_lock($oldPin != '' ? $oldPin : pinValue, pinValue, setPinEnabled);
						console.log('set_pin_lock response: ', response);
						goto('/security');
					} catch (error) {
						console.error('AUTHENITCATE_PIN ERROR: ', error);
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
