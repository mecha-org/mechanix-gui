<script lang="ts">
	import BlockItem from '$lib/components/block-item.svelte';
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListBlock from '$lib/components/list-block.svelte';
	import { goBack } from '$lib/services/common-services';
	import { fetchKnownNetworks, removeWifi } from '$lib/services/network-services';
	import { onMount } from 'svelte';

	import type { PageData } from '../../available/[network]/$types';
	import { knownNetworksList } from '$lib/stores/networkStore';
	export let data: PageData;
	let networkSSID: string = '';

	function formattitle(title: string) {
		let words = title.split(/[-\s]/);
		for (let i = 0; i < words.length; i++) {
			words[i] = words[i].charAt(0).toUpperCase() + words[i].slice(1);
		}
		return words.join(' ');
	}

	const removeClickHandler = async () => {
		console.log('removeClickHandler - networkSSID : ', networkSSID);
		try {
			await removeWifi(networkSSID);
			await fetchKnownNetworks();
			goBack();
		} catch (error) {
			// TODO: error handling in UI - show toast/popup
			console.log('removeClickHandler error: ', error);
		}
	};

	onMount(() => {
		networkSSID = $knownNetworksList?.find((x: any) => x.ssid == data.title)?.network_id!;
		console.log('manage network- known - ssid:  ', networkSSID);
	});
</script>

<Layout title={formattitle(data.title)}>
	<div class="mt-4 flex flex-col">
		{#each data.networkDetail as networkDetail}
			<ListBlock>
				{#each networkDetail as eachNetwork, index}
					<!-- isTopBorderVisible={index == 0}
				isBottomBorderVisible={index !== networkDetail.length - 1}	 -->
					<BlockItem borderY={true} title={eachNetwork.title}>
						<h2 class="text-misty-slate text-lg font-medium">
							{eachNetwork.value}
						</h2></BlockItem
					>
				{/each}
			</ListBlock>
		{/each}
	</div>
	<footer
		slot="footer"
		class="h-full w-full border-t-2 bg-[#05070A73] backdrop-blur-3xl backdrop-filter"
	>
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-1 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="left_arrow" width="60" height="60" />
			</button>
			<button
				class=" flex h-[60px] w-[60px] items-center justify-center rounded-lg p-1 text-[#FAFBFC]"
				on:click={removeClickHandler}
			>
				<Icons name="trash" width="60" height="60" />
			</button>
		</div>
	</footer>
</Layout>
