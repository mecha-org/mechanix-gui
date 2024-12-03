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
			<BlockItem title="OS" isBottomBorderVisible={true}>
				<p class="text-lg font-medium">Mechanix OS</p>
			</BlockItem>
			<BlockItem isBottomBorderVisible={true} title="Version">
				<p class="text-lg font-medium">24.01</p>
			</BlockItem>
		</ListBlock>
		<div class="p-6"></div>
		<ListBlock>
			<BlockItem title="Version" isBottomBorderVisible={true}>
				<p class="text-lg font-medium">24.01</p>
			</BlockItem>
			<BlockItem title="Serial Number" isBottomBorderVisible={true}>
				<p class="text-lg font-medium">123 123 123</p>
			</BlockItem>
			<BlockItem isBottomBorderVisible={true} title="Ethernet MAC Address">
				<p class="text-lg font-medium">B0:35:B5:DA:A6:75</p>
			</BlockItem>
		</ListBlock>
	</div>
	<footer
		slot="footer"
		class="border-silver-grayh-full w-full border-t-2 bg-[#05070A73] backdrop-blur-3xl backdrop-filter"
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
