<script lang="ts">
	import BlockItem from '$lib/components/block-item.svelte';
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListBlock from '$lib/components/list-block.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { goBack } from '$lib/services/common-services';
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import {
		wifiStatus,
		connectedNetwork,
		disableWifiSwitch,
		fetchingWifiStatus
	} from '$lib/stores/networkStore';
	import {
		fetchConnectedWifiInfo,
		fetchWifiStatus,
		type WirelessInfoResponse
	} from '$lib/services/network-services';

	const getInitalData = async () => {
		console.log('page::network::getInitalData()');
		try {
			let response = await fetchWifiStatus();
			fetchingWifiStatus.set(false);
			if (response) {
				fetchConnectedWifiInfo();
			} else {
				connectedNetwork.set({} as WirelessInfoResponse);
			}
		} catch (error) {
			console.error('page::network::getInitalData()::error:::: ', error);
		}
	};

	onMount(() => {
		getInitalData();
	});

	const onWifiStatuChangeHandler = async (flag: boolean) => {
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
			console.error('page::network::onWifiStatuChangeHandler()::error:::', error);
		}
	};
</script>

<Layout title="Network">
	<div class="flex flex-col gap-12">
		<div class="flex flex-col gap-4">
			<ListBlock>
				<BlockItem title="Enable Wireless" isBottomBorderVisible={!!$connectedNetwork.name}>
					{#if $fetchingWifiStatus}
						<div class="flex animate-spin flex-row items-center gap-2">
							<Icons name="spinner" height="30px" width="30px" />
						</div>
					{:else}
						<Switch
							bind:checked={$wifiStatus}
							onCheckedChange={onWifiStatuChangeHandler}
							disabled={$disableWifiSwitch}
						/>
					{/if}
				</BlockItem>
				{#if !!$connectedNetwork.name}
					<BlockItem
						isBottomBorderVisible={false}
						title={$connectedNetwork.name}
						href={`/network/manage-network/available/${$connectedNetwork.name}`}
					>
						<div class="flex flex-row items-center gap-4">
							<Icons height="30px" width="30px" name="blue_checked" />
							<Icons height="30px" width="30px" name="right_arrow" />
						</div>
					</BlockItem>
				{/if}
			</ListBlock>
			{#if $wifiStatus}
				<ListItem isLink href="/network/manage-network" title="Manage Networks"
					><Icons name="right_arrow" height="30px" width="30px" /></ListItem
				>
			{:else}
				<ListItem title="Manage Networks" isSelected={false}
					><Icons name="right_arrow" height="30px" width="30px" /></ListItem
				>
			{/if}
			<ListItem isLink href="/network/ip-settings" title="IP Settings"
				><Icons name="right_arrow" height="30px" width="30px" /></ListItem
			>
		</div>
		<div>
			<ListHeading title="Others" />
			<div class="flex flex-col gap-4">
				<ListItem isLink href="/network/ethernet" title="Ethernet"
					><Icons name="right_arrow" height="30px" width="30px" /></ListItem
				>
				<ListItem isLink href="/network/dns" title="DNS"
					><Icons name="right_arrow" height="30px" width="30px" /></ListItem
				>
			</div>
		</div>
	</div>
	<footer slot="footer" class="h-full w-full bg-[#05070A73] backdrop-blur-3xl backdrop-filter">
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="bg-ash-gray flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="right_arrow" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
