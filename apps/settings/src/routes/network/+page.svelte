<script lang="ts">
	import BlockItem from '$lib/components/block-item.svelte';
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListBlock from '$lib/components/list-block.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { LOG_LEVEL, consoleLog, goBack } from '$lib/services/common-services';
	import { invoke } from '@tauri-apps/api';
	import { onDestroy, onMount } from 'svelte';
	import {
		wifiStatus,
		connectedNetwork,
		disableWifiSwitch,
		fetchingWifiStatus,
		fetchingConnectedNetwork
	} from '$lib/stores/networkStore';
	import {
		fetchAvaialbleNetworks,
		fetchConnectedWifiInfo,
		fetchWifiStatus
	} from '$lib/services/network-services';
	import type { WirelessInfoResponse } from '$lib/types/NetworkTypes';
	import { ERROR_LOG, NETWORK_MODULE_LOG, PAGE_LOG, SET_INTERVAL_TIMER } from '../../constants';
	import { goto } from '$app/navigation';
	const LOG_PREFIX = PAGE_LOG + NETWORK_MODULE_LOG;

	let timeIntervalId: number;
	const getInitalData = async () => {
		consoleLog(LOG_PREFIX + 'getInitalData()::');
		try {
			let response = await fetchWifiStatus();
			if ($fetchingWifiStatus) {
				fetchingWifiStatus.set(false);
			}
			if (response) {
				fetchConnectedWifiInfo();
			} else {
				connectedNetwork.set({} as WirelessInfoResponse);
			}
		} catch (error) {
			consoleLog(LOG_PREFIX + 'getInitalData()::' + ERROR_LOG, {
				type: LOG_LEVEL.ERROR,
				data: error
			});
		}
	};

	onMount(() => {
		getInitalData();
		fetchAvaialbleNetworks();
		timeIntervalId = setInterval(getInitalData, SET_INTERVAL_TIMER);
	});

	onDestroy(() => {
		clearInterval(timeIntervalId);
	});

	const onWifiStatuChangeHandler = async (flag: boolean) => {
		consoleLog(LOG_PREFIX + 'onWifiStatuChangeHandler()::');
		try {
			disableWifiSwitch.set(true);
			if (flag) {
				const response: boolean = await invoke('enable_wifi');
				disableWifiSwitch.set(false);
				wifiStatus.set(response);
				if (response) {
					fetchConnectedWifiInfo();
				}
			} else {
				const response = await invoke('disable_wifi');
				if (response) {
					connectedNetwork.set({} as WirelessInfoResponse);
				}
				disableWifiSwitch.set(false);
			}
		} catch (error) {
			consoleLog(LOG_PREFIX + 'onWifiStatuChangeHandler()::' + ERROR_LOG, {
				type: LOG_LEVEL.ERROR,
				data: error
			});
		}
	};

	const addNewNetwork = () => {
		goto('/network/manage-network/new-network');
	};
</script>

<Layout title="Wireless" loader={$fetchingWifiStatus}>
	<div slot="switch">
		{#if $fetchingWifiStatus || $fetchingConnectedNetwork}
			<div class="flex animate-spin flex-row items-center gap-2">
				<Icons name="spinner" height="28px" width="28px" />
			</div>
		{:else}
			<Switch
				bind:checked={$wifiStatus}
				onCheckedChange={onWifiStatuChangeHandler}
				disabled={$disableWifiSwitch}
			/>
		{/if}
	</div>
	<div class="flex flex-col gap-12">
		<ListBlock>
			{#if !$fetchingConnectedNetwork || typeof $connectedNetwork.name !== 'undefined'}
				<BlockItem
					isBottomBorderVisible={false}
					title={$connectedNetwork.name}
					href={`/network/manage-network/available/${$connectedNetwork.name}?isConnected=${true}`}
					borderY={true}
				>
					<div class="flex flex-row items-center gap-4">
						<Icons height="24px" width="24px" name="blue_check_no_fill" />
						<Icons height="24px" width="24px" name="right_arrow" />
					</div>
				</BlockItem>
			{/if}
		</ListBlock>

		{#if $wifiStatus}
			<!-- <ListBlock>
				<BlockItem
					title="Manage Networks"
					isBottomBorderVisible={true}
					href="/network/manage-network"
				>
					<Icons name="right_arrow" height="30px" width="30px" />
				</BlockItem>
				<BlockItem
					title="Available Networks"
					isBottomBorderVisible={false}
					href="/network/available-network"
				>
					<Icons name="right_arrow" height="30px" width="30px" />
				</BlockItem>
			</ListBlock> -->
			<div class="flex flex-col">
				<ListItem isLink href="/network/manage-network" title="Manage Networks">
					<Icons name="right_arrow" height="30px" width="30px" />
				</ListItem>
				<ListItem isLink href="/network/available-network" title="Available Networks"
					><Icons name="right_arrow" height="30px" width="30px" />
				</ListItem>
			</div>
		{:else}
			<ListItem title="Manage Networks" isSelected={false}>
				<Icons name="right_arrow" height="30px" width="30px" />
			</ListItem>
		{/if}

		<!-- </div> -->
		<div>
			<ListHeading title="Other" />
			<div class="flex flex-col">
				<ListItem isLink href="/network/ip-settings" title="IP Settings">
					<Icons name="right_arrow" height="30px" width="30px" />
				</ListItem>
				<ListItem isLink href="/network/ethernet" title="Ethernet"
					><Icons name="right_arrow" height="30px" width="30px" /></ListItem
				>
				<ListItem isLink href="/network/dns" title="DNS"
					><Icons name="right_arrow" height="30px" width="30px" /></ListItem
				>
			</div>
		</div>
	</div>
	<footer
		slot="footer"
		class="border-silver-gray h-full w-full border-t-2 bg-[#05070A73] backdrop-blur-3xl backdrop-filter"
	>
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="  flex h-[60px] w-[60px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="left_arrow" width="60" height="60" />
			</button>
			<button
				class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={addNewNetwork}
			>
				<Icons name="addition" width="60" height="60" />
			</button>
		</div>
	</footer>
</Layout>
