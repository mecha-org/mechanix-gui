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
		disableBluetoothSwitch,
		isFetchingAvailableDevices,
		isFetchingOtherDevices,
		availableDevicesList,
		otherDevicesList
	} from '$lib/stores/bluetoothStore';
	import {
		disableBluetooth,
		enableBluetooth,
		fetchAvailableDevices,
		fetchBluetoothStatus,
		type BluetoothScanResponse
	} from '$lib/services/bluetooth-services';
	import { Label } from 'bits-ui';

	// $: code = '243562';
	$: code = undefined;

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

	const onBluetoothStatusChangeHandler = async (flag: boolean) => {
		console.log('onBluetoothStatusChangeHandler flag: ', flag);
		try {
			disableBluetoothSwitch.set(true);
			if (flag) {
				const response: boolean = await enableBluetooth();
				if (response) {
					await fetchAvailableDevices();
				}
			} else {
				const response: any = await disableBluetooth();
			}
		} catch (error) {
			console.error('page::bluetooth::onBluetoothStatusChangeHandler()::error:::', error);
		}
	};

	// NOTE: when api works, remove this code & check
	$: if (!$bluetoothStatus) {
		availableDevicesList.set([]);
		otherDevicesList.set([]);
	}
</script>

<Layout title="Bluetooth">
	<div slot="switch">
		{#if $fetchingBluetoothStatus}
			<div class="flex animate-spin flex-row items-center gap-2">
				<Icons name="spinner" height="28px" width="28px" />
			</div>
		{:else}
			<Switch
				bind:checked={$bluetoothStatus}
				onCheckedChange={onBluetoothStatusChangeHandler}
				disabled={$disableBluetoothSwitch}
			/>
		{/if}
	</div>
	<div class="mt-7">
		{#if $availableDevicesList.length > 0 || $isFetchingAvailableDevices}
			<ListHeading title="Available devices" />
		{/if}
		<div class="flex flex-col">
			{#if $isFetchingAvailableDevices}
				<ListItem title="Searching available devicess">
					<div class="flex animate-spin flex-row items-center gap-2">
						<Icons name="spinner" height="28px" width="28px" />
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
						<div class="flex flex-row items-center gap-3">
							{#if available_device?.is_trusted}
								<Icons name="blue_check_no_fill" height="24px" width="24px" />
							{/if}
							<Icons name="right_arrow" height="24px" width="24px" />
						</div>
					</ListItem>
				{/each}
			{/if}
		</div>
	</div>
	<div class="mt-7">
		<ListHeading title="Paired devices" />
		<div class="flex flex-col">
			{#if $isFetchingOtherDevices}
				<ListItem title="Searching paired devicess">
					<div class="flex animate-spin flex-row items-center gap-2">
						<Icons name="spinner" height="28px" width="28px" />
					</div>
				</ListItem>
			{:else if $otherDevicesList.length > 0}
				{#each $otherDevicesList as other_device}
					<ListItem
						isLink
						href={code
							? `/bluetooth/other-device/${other_device?.name?.trim().replace(/\s+/g, '-')}?address=${other_device?.address}&code=${code}`
							: `/bluetooth/other-device/${other_device?.name?.trim().replace(/\s+/g, '-')}?address=${other_device?.address}`}
						title={other_device?.name}
					>
						<div class="flex flex-row items-center gap-3">
							<Icons name="right_arrow" height="24px" width="24px" />
						</div>
					</ListItem>
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
		<div
			class="border-silver-gray flex h-full w-full flex-row items-center justify-between border-t-2 px-4 py-3"
		>
			<button
				class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-1 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="left_arrow" width="60" height="60" />
			</button>
			<button
				class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="addition" width="60" height="60" />
			</button>
		</div>
	</footer>
</Layout>
