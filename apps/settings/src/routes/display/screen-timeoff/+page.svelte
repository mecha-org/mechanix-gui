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

	let selected = { key: '10', title: '10s' };
	const isSelected = false;
	const submitHandler = () => {
		console.log('submitHandler : ', selected);
	};
</script>

<Layout title="Screen Off timeout">
	<div class="flex flex-col">
		<!-- <ListItem title="30s" isSelected>
			<Icons name="blue_radio_fill" height="30px" width="30px" />
		</ListItem> -->
		{#each intervals as interval}
			<button
				on:click={() => {
					selected.key = interval.key;
				}}
			>
				<ListItem title={interval.title}>
					{#if selected.key == interval.key}
						<Icons name="blue_radio_fill" height="30px" width="30px" />
					{:else}
						<Icons name="empty_ring" height="30px" width="30px" />
					{/if}
				</ListItem>
			</button>
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
				class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={submitHandler}
			>
				<Icons name="submit" width="60" height="60" />
			</button>
		</div>
	</footer>
</Layout>
