<script lang="ts">
	import BlockItem from '$lib/components/block-item.svelte';
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListBlock from '$lib/components/list-block.svelte';
	import { Switch } from '$lib/components/ui/switch';
	import { goBack } from '$lib/services/common-services';
	import { fetchKnownNetworks, removeWifi } from '$lib/services/network-services';
	import { onMount } from 'svelte';

	import type { PageData } from '../../available/[network]/$types';
	import { knownNetworksList } from '$lib/stores/networkStore';
	import { goto } from '$app/navigation';
	export let data: PageData;
	let networkSSID: string = '';

	function formattitle(title: string) {
		let words = title.split(/[-\s]/);
		for (let i = 0; i < words.length; i++) {
			words[i] = words[i].charAt(0).toUpperCase() + words[i].slice(1);
		}
		return words.join(' ');
	}

	onMount(() => {
		networkSSID = $knownNetworksList?.find((x: any) => x.ssid == data.title)?.network_id!;
		console.log('manage network- avilable - ssid:  ', networkSSID);
	});

	const removeClickHandler = async () => {
		console.log('removeClickHandler - networkSSID : ', networkSSID);
		try {
			await removeWifi(networkSSID);
			await fetchKnownNetworks();
		} catch (error) {
			// TODO: error handling in UI - show toast/popup
			console.log('removeClickHandler error: ', error);
		}
	};

	const backClickHandler = () => {
		goBack();
	}

	const addNetwork = () => {
		goto(`/network/manage-network/connect/${data.title}`);
	};
</script>

<Layout title={formattitle(data.title) + ` network details`}>
	<div class="flex flex-col gap-4">
		{#each data?.networkDetail as networkDetail}
			<ListBlock>
				{#each networkDetail as eachNetwork, index}
					{#if index == networkDetail.length - 1}<BlockItem
							isBottomBorderVisible={false}
							title={eachNetwork.title}
						>
							{#if typeof eachNetwork.value == 'boolean'}
								<Switch />
							{:else}
								<p class="text-misty-slate text-lg font-medium">{eachNetwork.value}</p>
							{/if}
						</BlockItem>
					{:else}
						<BlockItem title={eachNetwork.title}>
							{#if typeof eachNetwork.value == 'boolean'}
								<Switch />
							{:else}
								<p class="text-misty-slate text-lg font-medium">{eachNetwork.value}</p>
							{/if}
						</BlockItem>
					{/if}
				{/each}
			</ListBlock>
		{/each}
	</div>
	<footer slot="footer" class="h-full w-full bg-[#05070A73] backdrop-blur-3xl backdrop-filter">
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="bg-ash-gray flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={backClickHandler}
			>
				<Icons name="right_arrow" width="32" height="32" />
			</button>
			{#if data?.isConnected}
				<button
					class="bg-ash-gray flex h-[48px] w-[48px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
					on:click={removeClickHandler}
				>
					<Icons name="trash" width="32" height="32" />
				</button>
			<!-- {:else}
				<button
					class="bg-ash-gray flex h-[48px] w-[48px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
					on:click={addNetwork}
				>
					<Icons name="addition" width="32" height="32" />
				</button> -->
			{/if}
		</div>
	</footer>
</Layout>
