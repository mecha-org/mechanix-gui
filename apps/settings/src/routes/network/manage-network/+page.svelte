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
		<div class="flex flex-col gap-4">
			{#if $fetchingKnownNetworks}
				<ListItem title="Loading known networks">
					<div class="flex animate-spin flex-row items-center gap-2">
						<Icons name="spinner" height="30px" width="30px" />
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
								<Icons name="blue_checked" height="30px" width="30px" />

								<Icons name="lock" height="30px" width="30px" />
								<Icons name="network" height="30px" width="30px" />
								<Icons name="square_info" height="30px" width="30px" />
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
									<Icons name="spinner" height="30px" width="30px" />
								</div>
							{:else}
								<div class="flex flex-row items-center gap-2">
									<Icons name="lock" height="30px" width="30px" />
									<Icons name="network" height="30px" width="30px" />

									<a href={`/network/manage-network/known/${item.network_id}`}>
										<Icons name="square_info" height="30px" width="30px" />
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
	<footer slot="footer" class="h-full w-full bg-[#05070A73] backdrop-blur-3xl backdrop-filter">
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="bg-ash-gray flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="right_arrow" width="32" height="32" />
			</button>
			<!-- <button
				class="bg-ash-gray flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
			>
				<Icons name="addition" width="32" height="32" />
			</button> -->
		</div>
	</footer>
</Layout>
