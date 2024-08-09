<script>
	import BlockItem from '$lib/components/block-item.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListBlock from '$lib/components/list-block.svelte';
	import { goBack } from '$lib/services/common-services';
	import Icons from '$lib/components/icons.svelte';
</script>

<Layout title="About">
	<div class="flex flex-col gap-4">
		<ListBlock>
			<BlockItem title="OS">
				<p class="text-lg font-medium">Mechanix OS</p>
			</BlockItem>
			<BlockItem isBottomBorderVisible={false} title="Version">
				<p class="text-lg font-medium">24.01</p>
			</BlockItem>
		</ListBlock>
		<ListBlock>
			<BlockItem title="Version">
				<p class="text-lg font-medium">24.01</p>
			</BlockItem>
			<BlockItem title="Serial Number">
				<p class="text-lg font-medium">123 123 123</p>
			</BlockItem>
			<BlockItem isBottomBorderVisible={false} title="Ethernet MAC Address">
				<p class="text-lg font-medium">B0:35:B5:DA:A6:75</p>
			</BlockItem>
		</ListBlock>
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
