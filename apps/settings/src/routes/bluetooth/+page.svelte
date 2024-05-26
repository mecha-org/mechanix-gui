<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListBlock from '$lib/components/list-block.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { goBack } from '$lib/services/common-services';
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { bluetooth_store } from '$lib/stores';

	let bluetooth_data: any;
	let available_devices: any[] = [] || $bluetooth_store.available_devices;
	let other_devices: any[] = [];
	$: is_loading = false;
	$: checked = false;

	const check_bluetooth_data = async () => {
		console.log('check_bluetooth_data called...');
		is_loading = true;

		try {
			let response = await invoke('get_bluetooth_status');
			console.log('response: ', response);
			is_loading = false;
			bluetooth_data = response;

			if (bluetooth_data.status == 1) {
				checked = bluetooth_data.status == 1;
				available_devices = bluetooth_data.available_devices.filter((item: any) => {
					return item.is_paired || item.is_trusted;
				});
				other_devices = bluetooth_data.available_devices.filter((item: any) => {
					return !item.is_paired;
				});

				console.log('devices: ', { available_devices, other_devices });

				bluetooth_store.set({
					bluetooth_status: bluetooth_data.status,
					available_devices: available_devices,
					other_devices: other_devices
				});
			}
		} catch (error) {
			is_loading = false;
			console.error('check_bluetooth_data error : ', error);
		}
	};

	onMount(async () => {
		await check_bluetooth_data();
	});

	$: if (checked) {
		console.log('Enable Bluetooth!!!!');
		// timer();
	}
	// const enable_bluetooth = async() => {
	// 	is_loading = true;
	// 	try {
	// 		let response = await invoke('update_enable_bluetooth');
	// 		console.log('enable_bluetooth response: ', response);
	// 	} catch (error) {
	// 		is_loading = false;
	// 		console.error('enable_bluetooth error : ', error);
	// 	}
	// }

	// const timer = () => {
	// 	setTimeout(async () => {
	// 		await enable_bluetooth();
	// 	}, 500);

	// 	setInterval(async () => {
	// 		await check_bluetooth_data();
	// 	}, 10000);
	// };

	// const handleChange = async(e: boolean) => {
	// 	console.log('handleChange :: ', e);
	// 	// switch_toggle = e;
	// 	if (e == true) {
	// 		console.log("GET DATA!!!");
	// 		await check_bluetooth_data();
	// 	}
	// };

	// $: if (switch_toggle == true) check_bluetooth_data();
</script>

<Layout title="Bluetooth">
	<ListItem isLink title="Enable bluetooth">
		{#if is_loading}
			<div class="flex animate-spin flex-row items-center gap-2">
				<Icons name="spinner" height="30px" width="30px" />
			</div>
		{:else}
			<Switch bind:checked />
			<!-- <Switch bind:checked onCheckedChange={handleChange}/> -->
		{/if}
	</ListItem>
	<div class="mt-10">
		<ListHeading title="Available devices" />
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
		{:else}
			<!-- todo : add spinner  -->
			<ListItem
				isLink
				href={`/bluetooth/other-device/fetching-paired-devices`}
				title={'Fetching Available Device'}
			></ListItem>
		{/if}
	</div>
	<div class="mt-10">
		<ListHeading title="Paired devices" />
		{#if other_devices.length > 0}
			<div class="flex flex-col gap-4">
				{#each other_devices as other_device}
					<ListItem
						isLink
						href={`/bluetooth/other-device/${other_device?.name?.trim().replace(/\s+/g, '-')}`}
						title={other_device?.name}
					></ListItem>
				{/each}
			</div>
		{:else}
			<!-- todo : add spinner  -->
			<ListItem
				isLink
				href={`/bluetooth/other-device/fetching-paired-devices`}
				title={'Fetching Paired Device'}
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
