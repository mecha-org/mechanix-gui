<script lang="ts">
	import BlockItem from '$lib/components/block-item.svelte';
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListBlock from '$lib/components/list-block.svelte';
	import { Switch } from '$lib/components/ui/switch';
	import { goBack } from '$lib/services/common-services';

	import type { PageData } from '../../available/[network]/$types';
	export let data: PageData;
	function formattitle(title: string) {
		let words = title.split(/[-\s]/);
		for (let i = 0; i < words.length; i++) {
			words[i] = words[i].charAt(0).toUpperCase() + words[i].slice(1);
		}
		return words.join(' ');
	}
</script>

<Layout title={formattitle(data.title)}>
	<div class="flex flex-col gap-4">
		{#each data.networkDetail as networkDetail}
			<ListBlock>
				{#each networkDetail as eachNetwork, index}
					{#if index == networkDetail.length - 1}<BlockItem
							isBottomBorderVisible={false}
							title={eachNetwork.title}
						>
							{#if typeof eachNetwork.value == 'boolean'}
								<Switch />
							{:else}
								<p class="text-lg font-medium text-misty-slate">{eachNetwork.value}</p>
							{/if}
						</BlockItem>
					{:else}
						<BlockItem title={eachNetwork.title}>
							{#if typeof eachNetwork.value == 'boolean'}
								<Switch />
							{:else}
								<p class="text-lg font-medium text-misty-slate">{eachNetwork.value}</p>
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
				class="flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg bg-ash-gray p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="right_arrow" width="32" height="32" />
			</button>
			<button
				class="flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg bg-ash-gray p-2 text-[#FAFBFC]"
			>
				<Icons name="addition" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
