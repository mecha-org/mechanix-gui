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
	import { bluetoothStore } from '$lib/stores';


	let bluetooth_status: boolean = false;
	let available_devices: any[] = [] || $bluetoothStore.available_devices;
	let paired_devices: any[] = [];

	$: isStatusLoading = false;
	$: isScanLoading = false;
	$: checked = false; 

	const check_bluetooth_data = async () => {
		console.log('check_bluetooth_data called...');
		isStatusLoading = true;
		try {
			let response: any = await invoke('get_bluetooth_status');
			isStatusLoading = false;
			checked = response == 1 ?? false;  // for switch
			bluetooth_status = response;       // for store

			bluetoothStore.set({
				bluetooth_status: response,
			});
			console.log('response: ', { response, checked });
		} catch (error) {
			isStatusLoading = false;
			console.error('check_bluetooth_data error : ', error);
		}
	};

	// later: separate available vs paired device api
	const scan_bluetooth = async () => {
		console.log('scan_bluetooth called...');
		isScanLoading = true;
		try {
			let response: any = await invoke('scan_bluetooth');
			console.log('response: ', response);
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

	let unsubscribe = bluetoothStore.subscribe((value: any) => {
		console.log("mount un-subscribe here: ", value);
	});
	onDestroy(unsubscribe);


	const update_data = async() => {
		if(bluetooth_status || paired_devices.length == 0) {
			await check_bluetooth_data();
			await scan_bluetooth();
		}
	};

	setInterval(async () => {
		update_data();
	}, 15000);

	onMount(async () => {

		console.log("onMount: ", $bluetoothStore);
		console.log("onMount checkkk: ", Object.keys($bluetoothStore).length);

		if (Object.keys($bluetoothStore).length == 0) {
            update_data();
        }
		else {
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
			console.log('ENABLE BLUETOOTH!!!');
			enable_bluetooth();
		} else {
			console.log('DISABLE BLUETOOTH!!!');
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
			<!-- <Switch bind:checked /> -->
			<Switch bind:checked={bluetooth_status} onCheckedChange={handleChange} />
		{/if}
	</ListItem>
	<div class="mt-10">
		{#if available_devices.length > 0 || isScanLoading}
			<ListHeading title="Available devices" />
		{/if}

		{#if available_devices.length > 0}
			<div class="flex flex-col gap-4">
				{#each available_devices as { name, is_trusted }}
					<ListItem isLink href={`/bluetooth/${name?.trim().replace(/\s+/g, '-')}`} title={name}>
						<div class="flex flex-row items-center gap-4">
							<Icons name="blue_checked" height="30px" width="30px" />
							{#if is_trusted}<Icons name="right_arrow" height="30px" width="30px" />{/if}
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
						href={`/bluetooth/other-device/${other_device?.name?.trim().replace(/\s+/g, '-')}`}
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
				class="flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg bg-ash-gray p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="right_arrow" width="32" height="32" />
			</button>
			<button
				class="flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg bg-ash-gray p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="addition" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
