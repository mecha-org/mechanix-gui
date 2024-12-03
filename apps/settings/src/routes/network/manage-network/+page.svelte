<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { LOG_LEVEL, consoleLog, goBack } from '$lib/services/common-services';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onDestroy, onMount } from 'svelte';
	import {
		fetchAvaialbleNetworks,
		fetchConnectedWifiInfo,
		fetchKnownNetworks,
		removeWifi
	} from '$lib/services/network-services';
	import {
		changeKnownNetwork,
		fetchingKnownNetworks,
		knownNetworksList
	} from '$lib/stores/networkStore';
	import { ERROR_LOG, NETWORK_MODULE_LOG, PAGE_LOG, SET_INTERVAL_TIMER } from '../../../constants';
	const LOG_PREFIX = PAGE_LOG + NETWORK_MODULE_LOG + 'manage-network::';

	let timeIntervalId: number;

	const getInitalData = async () => {
		consoleLog(LOG_PREFIX + 'getInitalData()::');
		try {
			fetchKnownNetworks().finally(() => {
				if (fetchingKnownNetworks) {
					fetchingKnownNetworks.set(false);
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
		changeKnownNetwork.set(true);
		consoleLog(LOG_PREFIX + 'connectedToNetwork()::' + networkSSID);
		try {
			const response: boolean = await invoke('connect_to_known_network', {
				networkSsid: networkSSID
			});
			await fetchConnectedWifiInfo();
			changeKnownNetwork.set(false);
		} catch (error) {
			changeKnownNetwork.set(false);

			console.log(LOG_PREFIX + 'connectedToNetwork()::error::', error);
		}
	};
</script>

<Layout title="Manage Network">
	<!-- <ListHeading title="Known Networks" /> -->
	<div class="flex flex-col gap-12">
		<div class="flex flex-col">
			{#if $fetchingKnownNetworks}
				<ListItem title="Loading known networks">
					<div class="flex animate-spin flex-row items-center gap-2">
						<Icons name="spinner" height="28px" width="28px" />
					</div>
				</ListItem>
			{:else if $knownNetworksList.length > 0}
				{#each $knownNetworksList as item, i (item.network_id)}
					{#if item.flags.includes('CURRENT')}
						<ListItem
							isLink
							href={`/network/manage-network/known/${item.network_id}`}
							title={item.ssid}
						>
							<div class="flex flex-row items-center gap-2">
								<Icons name="blue_check_no_fill" height="24px" width="24px" />

								<Icons name="lock" height="24px" width="24px" />
								<Icons name="network" height="24px" width="24px" />
								<Icons name="square_info" height="24px" width="24px" />
							</div>
						</ListItem>
					{:else}
						<ListItem
							isSelected
							title={item.ssid}
							isLink={false}
							on:click={() => connectedToNetwork(item.network_id)}
						>
							{#if $changeKnownNetwork}
								<div class="flex animate-spin flex-row items-center gap-2">
									<Icons name="spinner" height="28px" width="28px" />
								</div>
							{:else}
								<div class="flex flex-row items-center gap-2">
									<Icons name="lock" height="24px" width="24px" />
									<Icons name="network" height="24px" width="24px" />

									<a href={`/network/manage-network/known/${item.network_id}`}>
										<Icons name="square_info" height="24px" width="24px" />
									</a>
								</div>
							{/if}
						</ListItem>
					{/if}
				{/each}
			{:else}
				<ListItem title="No networks available"></ListItem>
			{/if}
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
		</div>
	</footer>
</Layout>
