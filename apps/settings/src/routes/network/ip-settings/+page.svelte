<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { goBack } from '$lib/services/common-services';
	let selectedSetting = 'dhcp';
</script>

<Layout title="IP Settings">
	<div class="flex flex-col gap-4">
		<button
			on:click={() => {
				selectedSetting = 'dhcp';
			}}
		>
			<ListItem isSelected={selectedSetting == 'dhcp'} title="Automatic (DHCP)">
				{#if selectedSetting == 'dhcp'}
					<Icons name="blue_checked" height="30px" width="30px" />
				{:else}
					<Icons name="empty_ring" height="30px" width="30px" />
				{/if}
			</ListItem>
		</button>
		<button
			on:click={() => {
				selectedSetting = 'static';
			}}
		>
			<ListItem isSelected={selectedSetting == 'static'} title="Static">
				{#if selectedSetting == 'static'}
					<Icons name="blue_checked" height="30px" width="30px" />
				{:else}
					<Icons name="empty_ring" height="30px" width="30px" />
				{/if}
			</ListItem>
		</button>
		{#if selectedSetting == 'static'}
			<div class="flex flex-col gap-4">
				<div>
					<ListHeading title="IP Address" />
					<Input placeholder="Type here" />
				</div>
				<div>
					<ListHeading title="Gateway" />
					<Input placeholder="Type here" />
				</div>
			</div>
		{/if}
	</div>
	<footer slot="footer" class="h-full w-full bg-[#05070A73] backdrop-blur-3xl backdrop-filter">
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg bg-ash-gray p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="right_arrow" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
