<script lang="ts">
	import { goto } from '$app/navigation';
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { goBack } from '$lib/services/common-services';
	import {
		authenticate_pin,
		get_lock_status,
		remove_pin_lock,
		set_pin_lock
	} from '$lib/services/security-service';
	import Input from '$lib/components/ui/input/input.svelte';
	import * as Dialog from '$lib/components/ui/dialog';

	import {
		disableLockSwitch,
		fetchingLockStatus,
		currentLockStatus,
		ChangePinTypes,
		ChangePinTypesInfo,
		maxPinLength,
		minPinLength,
		oldPin
	} from '$lib/stores/securityStore';
	import { onMount } from 'svelte';
	export let pinValue: string = '';
	export let showError: boolean = false;
	export let errorMessage: string = '';

	let openPinModal: boolean = false;
	let setNewSecret: boolean = false;
	let modalType: ChangePinTypes;

	let currentSwitchStatus : boolean = false;
	$: changePinClick = false;


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
		{ value: 'backPress', icon: 'back' as string }, // back
		{ value: '9' },
		{ value: '0' },
		{ value: 'enterPress', icon: 'blue_tick' as string } // enter
	];

	const getInitalData = async () => {
		await get_lock_status();
	 	currentSwitchStatus = $currentLockStatus;
	};

	const enableLockHandler = (flag: boolean) => {
		console.log('enableLockHandler: ', flag, $currentLockStatus);

		if (flag) {
			setNewSecret = true;
			modalType = ChangePinTypes.SET_PIN;
			showModal(true);
		} else {
			modalType = ChangePinTypes.AUTHENTICATE_PIN;
			setNewSecret = false;
			showModal(true);
		}
	};

	const changePinClickHandler = () => {
		console.log('changePinClickHandler...', $currentLockStatus);

		// TODO: fix 1st time enabling pin does not change page
		if ($currentLockStatus) {
			changePinClick = true;
			modalType = ChangePinTypes.AUTHENTICATE_PIN;
			showModal(true);
		}
	};

	const keyClickHandler = (key: string) => async () => {
		console.log('keyClickHandler : ', { modalType }, $currentLockStatus);
		let updatePin: string = '';
		if (updatePin.length != maxPinLength) updatePin = pinValue + key;

		switch (key) {
			case 'backPress':
				currentSwitchStatus = $currentLockStatus == true;
				pinValue = '';
				showModal(false);
				break;

			case 'enterPress':
				console.log('ENTER API CALL', { pinValue, modalType });
				if (pinValue.length > maxPinLength || pinValue.length < minPinLength) {
					// TODO : toast
					console.log('TOAST: PIN must be 4-8 digits');
					return;
				}

				console.log("DIALOG: ", {modalType, setNewSecret, changePinClick});

				if (modalType == ChangePinTypes.SET_PIN) {
					try {
						const response = set_pin_lock(
							$oldPin != '' ? $oldPin : pinValue,
							pinValue,
							setNewSecret
						);
						setNewSecret = false; // IMP
						console.log('set_pin_lock response: ', response);
						pinValue = '';
						showModal(false);
					} catch (error) {
						console.error('AUTHENITCATE_PIN ERROR: ', error);
					}
				} else if (modalType == ChangePinTypes.AUTHENTICATE_PIN) {
					try {
						const response = await authenticate_pin(pinValue);
						console.log('authenticate_pin response: ', response);

						if (!response) {
							showError = true;
							errorMessage = 'Pin is incorrect, retry!';
							pinValue = '';
						} else {
							if (setNewSecret || changePinClick) {
								pinValue = '';
								modalType = ChangePinTypes.SET_PIN;
								showModal(true);
							} else {
								remove_pin_lock(pinValue);
								showModal(false);
							}
						}

						setTimeout(() => {
							showError = false;
						}, 2000);
					} catch (error) {
						console.error('AUTHENTICATE_PIN ERROR: ', error);
					}
				}
				break;

			default:
				if (updatePin.length <= maxPinLength) {
					pinValue = updatePin;
				}
				break;
		}
	};

	const erasePinCharacter = () => {
		if (pinValue !== '') pinValue = pinValue.slice(0, -1);
		console.log('updated pinValue: ', pinValue);
	};

	const showModal = (state: boolean) => {
		openPinModal = state;
	};

	onMount(() => {
		getInitalData();
	});
</script>

<Layout title="Security">
	<div class="flex flex-col gap-4">
		<ListItem title="Enable lock" isLink>
			{#if $fetchingLockStatus}
				<div class="flex animate-spin flex-row items-center gap-2">
					<Icons name="spinner" height="30px" width="30px" />
				</div>
			{:else}
				<Switch
					bind:checked={currentSwitchStatus}
					onCheckedChange={enableLockHandler}
					disabled={$disableLockSwitch}
				/>
			{/if}
		</ListItem>
		<ListItem title="Lock timeout" isLink href="/security/lock-timeout">
			<div class="flex flex-row items-center gap-2">
				<p class="text-lg text-misty-slate">10s</p>
				<Icons name="right_arrow" height="30px" width="30px" />
			</div>
		</ListItem>

		{#if $currentLockStatus}
			<button
				class="mt-4 flex h-[62px] w-full items-center justify-center rounded-lg bg-[#2F2F39] text-xl font-medium hover:bg-[#2F2F39]/80"
				on:click={changePinClickHandler}
			>
				Change pin
			</button>
		{/if}

		<Dialog.Root
			open={openPinModal}
			onOutsideClick={(e) => {
				e.preventDefault();
			}}
		>
			<Dialog.Content class="h-[70%] w-[70%] rounded-lg border-0 bg-[#15171D;]">
				<Dialog.Header class="">
					<Dialog.Title class="flex justify-center">
						{ChangePinTypesInfo[modalType].title}
					</Dialog.Title>
				</Dialog.Header>

				<Dialog.Description class="h-full text-white">
					{#if showError}
						<div
							class="flex animate-pulse items-center justify-center text-lg normal-case text-gray-400"
						>
							{errorMessage}
						</div>
					{/if}
					<div class="flex items-center justify-center">
						<Input
							class="text-center text-xl"
							placeholder="Enter pin"
							type="password"
							minlength={minPinLength}
							maxlength={maxPinLength}
							bind:value={pinValue}
						/>
						<button
							class=" flex h-[48px] w-[48px] items-center justify-center p-2 text-[#FAFBFC]"
							on:click={erasePinCharacter}
						>
							<Icons name="backspace" width="32" height="32" />
						</button>
					</div>
					<div class="flex flex-1 items-center justify-center">
						<div class="grid grid-cols-4 gap-4 p-4">
							{#each keysArray as key (key?.value)}
								<button
									class="rounded-md bg-[#2C2F36] px-2.5 py-1.5 text-2xl font-bold text-white"
									on:click={keyClickHandler(key?.value)}
								>
									{#if key.icon && (key.value == 'backPress' || key.value == 'enterPress')}
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
	<footer slot="footer" class="h-full w-full bg-[#05070A73] backdrop-blur-3xl backdrop-filter">
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg bg-ash-gray p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="right_arrow" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
