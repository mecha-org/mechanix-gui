<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { goBack } from '$lib/services/common-services';
	const intervals = [
		{ key: '10', title: '10s' },
		{ key: '30', title: '30s' },
		{ key: '60', title: '60s' },
		{ key: '300', title: '5m' },
		{ key: '600', title: '10m' }
	];
	const isSelected = false;
</script>

<Layout title="Screen Off timeout">
	<div class="flex flex-col">
		<ListItem title="30s" isSelected>
			<Icons name="blue_radio_fill" height="30px" width="30px" />
		</ListItem>
		{#each intervals as interval}
			<ListItem title={interval.title}>
				{#if false}
					<Icons name="blue_radio_fill" height="30px" width="30px" />
				{:else}
					<Icons name="empty_ring" height="30px" width="30px" />
				{/if}
			</ListItem>
		{/each}
	</div>
	<footer slot="footer" class="h-full w-full bg-[#05070A73] backdrop-blur-3xl backdrop-filter">
		<div
			class="border-silver-gray flex h-full w-full flex-row items-center justify-between border-t-2 px-4 py-3"
		>
			<button
				class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-1 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="left_arrow" width="60" height="60" />
			</button>
		</div>
	</footer>
</Layout>
