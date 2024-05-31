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
	import {
		bluetoothStatus,
		fetchingBluetoothStatus,
		isFetchingAvailableDevices,
		isFetchingOtherDevices,
		availableDevicesList,
		otherDevicesList
	} from '$lib/stores/bluetoothStore';
	import {
		fetchAvailableDevices,
		fetchBluetoothStatus,
		type BluetoothScanResponse
	} from '$lib/services/bluetooth-services';

	const getInitalData = async () => {
		console.log('page::bluetooth::getInitalData()');
		try {
			let response = await fetchBluetoothStatus();
			fetchingBluetoothStatus.set(false);
			if (response) {
				await fetchAvailableDevices();
				isFetchingAvailableDevices.set(false);
				isFetchingOtherDevices.set(false);
			}
		} catch (error) {
			console.error('page::bluetooth::getInitalData()::error:::: ', error);
		}
	};

	onMount(() => {
		getInitalData();
	});

	onDestroy(() => {
		console.log('ON DESTROY');
	});

	// setInterval(async () => {
	// 	await update_data();
	// }, 15000);

	const onBluetoothStatusChangeHandler = async (flag: boolean) => {
		try {
			if (flag) {
				const response: boolean = await invoke('enable_bluetooth');
				bluetoothStatus.set(response);
				if (response) {
					await fetchAvailableDevices();
				}
			} else {
				const response = await invoke('disable_bluetooth');
				if (response) {
					availableDevicesList.set([] as BluetoothScanResponse[]);
					otherDevicesList.set([] as BluetoothScanResponse[]);
				}
			}
		} catch (error) {
			console.error('page::bluetooth::onBluetoothStatusChangeHandler()::error:::', error);
		}
	};
</script>

<Layout title="Bluetooth">
	<ListItem isLink title="Enable bluetooth">
		{#if $fetchingBluetoothStatus}
			<div class="flex animate-spin flex-row items-center gap-2">
				<Icons name="spinner" height="30px" width="30px" />
			</div>
		{:else}
			<Switch
				bind:checked={$bluetoothStatus}
				onCheckedChange={onBluetoothStatusChangeHandler}
			/>
		{/if}
	</ListItem>
	<div class="mt-10">
		{#if $availableDevicesList.length > 0 || $isFetchingAvailableDevices}
			<ListHeading title="Available devices" />
		{/if}
		<div class="flex flex-col gap-4">
			<!-- <ListItem isLink href="/bluetooth/Ritika's-mecha-compute" title="Ritika's mecha compute">
				<div class="flex flex-row items-center gap-4">
					<Icons name="blue_checked" height="30px" width="30px" />
					<Icons name="right_arrow" height="30px" width="30px" />
				</div>
			</ListItem>
			<ListItem isLink href="/network/dns" title="Ritika's mecha compute 2"
				><Icons name="right_arrow" height="30px" width="30px" /></ListItem
			> -->
			{#if $isFetchingAvailableDevices}
				<ListItem title="Searching available devicess">
					<div class="flex animate-spin flex-row items-center gap-2">
						<Icons name="spinner" height="30px" width="30px" />
					</div>
				</ListItem>
			{:else}
				{#each $availableDevicesList as available_device}
					<ListItem
						isLink
						href={`/bluetooth/${available_device?.name?.trim().replace(/\s+/g, '-')}?address=${available_device?.address}&type=${available_device?.icon}`}
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
			{/if}
		</div>
	</div>
	<div class="mt-10">
		<ListHeading title="Paired devices" />
		<div class="flex flex-col gap-4">
			{#if $isFetchingOtherDevices}
				<ListItem title="Searching paired devicess">
					<div class="flex animate-spin flex-row items-center gap-2">
						<Icons name="spinner" height="30px" width="30px" />
					</div>
				</ListItem>
			{:else if $otherDevicesList.length > 0}
				{#each $otherDevicesList as other_device}
					<ListItem
						isLink
						href={`/bluetooth/other-device/${other_device?.name?.trim().replace(/\s+/g, '-')}?address=${other_device?.address}`}
						title={other_device?.name}
					></ListItem>
				{/each}
			{:else}
				<ListItem
					isLink={false}
					isSelected={false}
					href={`/bluetooth/other-device/searching-paired-devices`}
					title={$isFetchingAvailableDevices ? 'Searching available devices' : 'No Device Found'}
				/>
			{/if}
		</div>
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
