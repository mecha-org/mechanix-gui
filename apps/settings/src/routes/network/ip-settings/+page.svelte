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
	<div class="flex flex-col">
		<button
			on:click={() => {
				selectedSetting = 'dhcp';
			}}
		>
			<ListItem isSelected={selectedSetting == 'dhcp'} title="Automatic (DHCP)">
				{#if selectedSetting == 'dhcp'}
					<Icons name="blue_radio_fill" height="24px" width="24px" />
				{:else}
					<Icons name="empty_ring" height="24px" width="24px" />
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
					<Icons name="blue_radio_fill" height="24px" width="24px" />
				{:else}
					<Icons name="empty_ring" height="24px" width="24px" />
				{/if}
			</ListItem>
		</button>
		{#if selectedSetting == 'static'}
			<div class="flex flex-col">
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
	<footer
		slot="footer"
		class="border-silver-gray h-full w-full border-t-2 bg-[#05070A73] backdrop-blur-3xl backdrop-filter"
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
