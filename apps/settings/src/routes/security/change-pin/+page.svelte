<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import { goBack } from '$lib/services/common-services';
	import Input from '$lib/components/ui/input/input.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import DialogOverlay from '$lib/components/ui/dialog/dialog-overlay.svelte';
	import { page } from '$app/stores';
	import {
		ChangePinTypesInfo,
		ChangePinTypes,
		selectedPinLength,
	} from '$lib/stores/securityStore';
	import { authenticate_pin, set_pin } from '$lib/services/security-service';

	// let pinValue: string = '';
	$: pinValue = '';
	const state: any = $page.state;
	const screenType: ChangePinTypes = state?.screenType;

	console.log('CHANGE PIN state: ', { state, screenType }, typeof screenType);

	type KeysType = {
		value: string;
		icon?: string;
	};
	let keysArray: KeysType[] = [
		{ value: '1' },
		{ value: '2' },
		{ value: '3' },
		{ value: '4' },
		{ value: '5' },
		{ value: '6' },
		{ value: '7' },
		{ value: '8' },
		{ value: 'cancel', icon: 'cancel' as string }, // back
		{ value: '9' },
		{ value: '0' },
		{ value: 'backspace', icon: 'backspace' as string } // backspace - erase
	];

	const keyClickHandler = (key: string) => async () => {
		console.log('keyClickHandler: ', key);
		console.log('default case LENGTH CEHCK: ', selectedPinLength, pinValue.length);
		let updatePin = pinValue + key;

		switch (key) {
			case 'cancel':
				goBack();
				break;

			case 'backspace':
				if (pinValue !== '') pinValue = pinValue.slice(0, -1);
				console.log('updated pinValue: ', pinValue);
				break;

			default:
				if (updatePin.length < selectedPinLength) {
					pinValue = updatePin;
				} else {
					if (screenType == ChangePinTypes.SET_PIN) {
						try {
							const response = await authenticate_pin(pinValue);
							console.log('authenticate_pin response: ', response);
							goBack();
						} catch (error) {
							console.error('AUTHENITCATE_PIN ERROR: ', error);
						}
					} else if (screenType == ChangePinTypes.AUTHENTICATE_PIN) {
						try {
							const response = await set_pin(pinValue);
							console.log('set_pin response: ', response);
							// temp - get locak status on previous page
							// disable switch - false 
							// enable switch - true

							goBack();
						} catch (error) {
							console.error('SET_PIN ERROR: ', error);
						}
					}
				}
				break;
		}
	};
</script>

<Layout title="">
	<div class="flex gap-4">
		<Dialog.Root
			open={true}
			onOutsideClick={(e) => {
				e.preventDefault();
			}}
		>
			<Dialog.Content class="h-[70%] w-[70%] rounded-lg border-0 bg-[#15171D;]">
				<Dialog.Header class="">
					<Dialog.Title class="flex justify-center">
						{ChangePinTypesInfo[screenType].title}
					</Dialog.Title>
				</Dialog.Header>

				<Dialog.Description class="h-full text-white">
					<div class="flex flex-1 items-center justify-center">
						<Input class="text-xl text-center" placeholder="Enter pin" type="password" bind:value={pinValue} />
					</div>
					<div class="flex flex-1 items-center justify-center">
						<div class="grid grid-cols-4 gap-4 p-4">
							{#each keysArray as key (key?.value)}
								<button
									class="rounded-md bg-[#2C2F36] px-2.5 py-1.5 text-2xl font-bold text-white"
									on:click={keyClickHandler(key.value)}
								>
									{#if key.icon && (key.value == 'cancel' || key.value == 'backspace')}
										<Icons name={key.icon} height="30px" width="30px" />
									{:else}
										{key.value}
									{/if}
								</button>
							{/each}
						</div>
					</div>
				</Dialog.Description>
			</Dialog.Content>
		</Dialog.Root>
	</div>
</Layout>
