<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListBlock from '$lib/components/list-block.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { goBack } from '$lib/services/common-services';
	import { invoke } from '@tauri-apps/api';
	import { onDestroy, onMount } from 'svelte';
	import { bluetoothStore, checkUpdate } from '$lib/stores';

	console.log('STORE: ', $checkUpdate);

	let bluetooth_status: boolean = $bluetoothStore?.bluetooth_status || false;
	let available_devices: any[] = $bluetoothStore?.available_devices || [];
	let paired_devices: any[] = $bluetoothStore?.paired_devices || [];

	$: isStatusLoading = false;
	$: isScanLoading =  false;

	const check_bluetooth_data = async () => {
		console.log('check_bluetooth_data called...');
		isStatusLoading = true;
		try {
			let response: any = await invoke('get_bluetooth_status');
			isStatusLoading = false;
			bluetooth_status = response; // for switch

			bluetoothStore.set({
				bluetooth_status: response
			});
			console.log('status response: ', response );
		} catch (error) {
			isStatusLoading = false;
			console.error('check_bluetooth_data error : ', error);
		}
	};

	// later: separate available vs paired device api
	const scan_bluetooth = async () => {
		isScanLoading= true;
		try {
			let response: any = await invoke('scan_bluetooth');
			console.log('scan_bluetooth response: ', response);
			isScanLoading = false;

			if (response.length > 0) {
				available_devices = response.filter((item: any) => {
					return item.is_paired || item.is_trusted;
				});
				paired_devices = response.filter((item: any) => {
					return !item.is_paired;
				});

				bluetoothStore.set({
					bluetooth_status: bluetooth_status,
					available_devices: available_devices,
					paired_devices: paired_devices
				});

				console.log('devices: ', { available_devices, paired_devices });
			}
		} catch (error) {
			isScanLoading = false;
			console.error('scan_bluetooth error : ', error);
		}
	};

	// let unsubscribe = bluetoothStore.subscribe((value: any) => {
	// 	console.log('mount un-subscribe here: ', value);
	// });
	// onDestroy(unsubscribe);

	const update_data = async () => {
		if (bluetooth_status || paired_devices.length == 0) {
			await check_bluetooth_data();
			await scan_bluetooth();
		}
	};

	// setInterval(async () => {
	// 	await update_data();
	// }, 15000);

	onMount(async () => {
		if (Object.keys($bluetoothStore).length == 0 || $checkUpdate) {
			await update_data();
		} else {
			bluetooth_status = $bluetoothStore.bluetooth_status!;
			available_devices = $bluetoothStore.available_devices!;
			paired_devices = $bluetoothStore.paired_devices!;
		}
	});

	const disable_bluetooth = async () => {
		try {
			let response = await invoke('update_disable_bluetooth');
			console.log('disable_bluetooth response: ', response);
			available_devices = [];
			paired_devices = [];

			bluetoothStore.set({
				bluetooth_status: false,
				available_devices: available_devices,
				paired_devices: paired_devices
			});
		} catch (error) {
			console.error('disable_bluetooth error : ', error);
		}
	};

	const enable_bluetooth = async () => {
		try {
			let response = await invoke('update_enable_bluetooth');
			console.log('enable_bluetooth response: ', response);
			bluetooth_status = true;
			update_data();
		} catch (error) {
			console.error('enable_bluetooth error : ', error);
		}
	};

	const handleChange = async (e: boolean) => {
		console.log('handleChange :: ', e);
		if (e == true) {
			enable_bluetooth();
		} else {
			disable_bluetooth();
		}
	};
</script>

<Layout title="Bluetooth">
	<ListItem isLink title="Enable bluetooth">
		{#if isStatusLoading}
			<div class="flex animate-spin flex-row items-center gap-2">
				<Icons name="spinner" height="30px" width="30px" />
			</div>
		{:else}
			<Switch bind:checked={bluetooth_status} onCheckedChange={handleChange} />
		{/if}
	</ListItem>
	<div class="mt-10">
		{#if available_devices.length > 0 || isScanLoading}
			<ListHeading title="Available devices" />
		{/if}

		{#if available_devices.length > 0}
			<div class="flex flex-col gap-4">
				{#each available_devices as available_device}
					<ListItem
						isLink
						href={`/bluetooth/${available_device?.name?.trim().replace(/\s+/g, '-')}?address=${available_device?.address}`}
						title={available_device?.name}
						isSelected={available_device?.is_trusted ?? false}
					>
						<div class="flex flex-row items-center gap-4">
							{#if available_device?.is_trusted}
								<Icons name="blue_checked" height="30px" width="30px" />
							{/if}
							<Icons name="right_arrow" height="30px" width="30px" />
						</div>
					</ListItem>
				{/each}
			</div>
		{:else if isScanLoading && available_devices.length == 0}
			<ListItem
				isLink={false}
				isSelected={false}
				isLoading={isScanLoading}
				href={`/bluetooth/other-device/searching-paired-devices`}
				title={isScanLoading ? 'Searching available devices' : 'No Device Found'}
			></ListItem>
		{/if}
	</div>
	<div class="mt-10">
		<ListHeading title="Paired devices" />
		{#if paired_devices.length > 0}
			<div class="flex flex-col gap-4">
				{#each paired_devices as other_device}
					<ListItem
						isLink
						href={`/bluetooth/other-device/${other_device?.name?.trim().replace(/\s+/g, '-')}?address=${other_device?.address}`}
						title={other_device?.name}
					></ListItem>
				{/each}
			</div>
		{:else}
			<ListItem
				isLink={false}
				isSelected={false}
				isLoading={isScanLoading}
				href={`/bluetooth/other-device/searching-paired-devices`}
				title={isScanLoading ? 'Searching paired devices' : 'No Device Found'}
			></ListItem>
		{/if}
	</div>
	<footer slot="footer" class="h-full w-full bg-[#05070A73] backdrop-blur-3xl backdrop-filter">
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="bg-ash-gray flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="right_arrow" width="32" height="32" />
			</button>
			<button
				class="bg-ash-gray flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="addition" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
