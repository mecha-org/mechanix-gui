<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { LOG_LEVEL, consoleLog, goBack } from '$lib/services/common-services';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onDestroy, onMount } from 'svelte';
	import { fetchAvaialbleNetworks, fetchConnectedWifiInfo } from '$lib/services/network-services';
	import {
		availableNetworksList,
		fetchingAvailableNetworks,
		knownNetworksList
	} from '$lib/stores/networkStore';
	import { ERROR_LOG, NETWORK_MODULE_LOG, PAGE_LOG, SET_INTERVAL_TIMER } from '../../../constants';
	import { goto } from '$app/navigation';
	import Warning from 'postcss/lib/warning';
	const LOG_PREFIX = PAGE_LOG + NETWORK_MODULE_LOG + 'manage-network::';

	let timeIntervalId: number;
	const getInitalData = async () => {
		consoleLog(LOG_PREFIX + 'getInitalData()::');
		try {
			fetchAvaialbleNetworks().finally(() => {
				if (fetchingAvailableNetworks) {
					fetchingAvailableNetworks.set(false);
				}
			});
		} catch (error) {
			consoleLog(LOG_PREFIX + 'getInitalData()::' + ERROR_LOG, {
				type: LOG_LEVEL.ERROR,
				data: error
			});
		}
	};

	onMount(() => {
		getInitalData();
		timeIntervalId = setInterval(getInitalData, SET_INTERVAL_TIMER);
	});

	onDestroy(() => {
		clearInterval(timeIntervalId);
	});

	const connectedToNetwork = async (networkSSID: string) => {
		consoleLog(LOG_PREFIX + 'connectedToNetwork()::' + networkSSID);
		try {
			const response: boolean = await invoke('connect_to_known_network', {
				networkSsid: networkSSID
			});
			fetchConnectedWifiInfo();
		} catch (error) {
			console.log(LOG_PREFIX + 'connectedToNetwork()::error::', error);
		}
	};

	$: availableNetworksToShow = $availableNetworksList.filter(
		(network) =>
			!$knownNetworksList.some((i) => i.ssid == network.name || network.name.includes('\\'))
	);

	console.log('availableNetworksToShow: ', $availableNetworksList);
	const backClickHandler = () => {
		// goto(`/network`);
		goBack();
	};
</script>

<Layout title="Available Networks">
	<div class="flex flex-col gap-12">
		<div class="flex flex-col">
			{#if $fetchingAvailableNetworks}
				<ListItem title="Searching available networks">
					<div class="flex animate-spin flex-row items-center gap-2">
						<Icons name="spinner" height="30px" width="30px" />
					</div>
				</ListItem>
			{:else if availableNetworksToShow.length > 0}
				{#each availableNetworksToShow as item, i (item.name)}
					<ListItem isLink href={`/network/manage-network/connect/${item.name}`} title={item.name}>
						<div class="flex flex-row items-center gap-2">
							{#if item?.isSecured}
								<Icons name="lock" height="24px" width="24px" />
							{:else}
								<Icons name="warning" height="24px" width="24px" />
							{/if}
							<Icons name="network" height="24px" width="24px" />
							<a href={`/network/manage-network/available/${item.name}`}>
								<Icons name="square_info" height="24px" width="24px" />
							</a>
						</div>
					</ListItem>
				{/each}
			{:else}
				<ListItem title="No networks available"></ListItem>
			{/if}
		</div>
	</div>
	<footer
		slot="footer"
		class="border-silver-grayh-full w-full border-t-2 bg-[#05070A73] backdrop-blur-3xl backdrop-filter"
	>
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="  flex h-[60px] w-[60px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={backClickHandler}
			>
				<Icons name="left_arrow" width="60" height="60" />
			</button>
		</div>
	</footer>
</Layout>
