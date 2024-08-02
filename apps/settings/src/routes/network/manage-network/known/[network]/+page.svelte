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
	let networkSSID : string = '';

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
			console.log("removeClickHandler error: ", error);
		}
	};

	onMount(() => {
		networkSSID = $knownNetworksList?.find((x: any) => x.ssid == data.title)?.network_id!;
		console.log('manage network- known - ssid:  ', networkSSID);
	});
</script>

<Layout title={formattitle(data.title)+` network details`}>
	<div class="flex flex-col gap-4">
		{#each data.networkDetail as networkDetail}
			<ListBlock>
				{#each networkDetail as eachNetwork, index}
					<BlockItem
						isBottomBorderVisible={index !== networkDetail.length - 1}
						title={eachNetwork.title}
					>
						<h2 class="text-lg font-medium text-misty-slate">
							{eachNetwork.value}
						</h2></BlockItem
					>
				{/each}
			</ListBlock>
		{/each}
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
				class="bg-ash-gray flex h-[48px] w-[48px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={removeClickHandler}
			>
				<Icons name="trash" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
