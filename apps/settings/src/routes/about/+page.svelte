<script>
	import BlockItem from '$lib/components/block-item.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListBlock from '$lib/components/list-block.svelte';
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
</Layout>
