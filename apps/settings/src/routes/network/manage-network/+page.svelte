<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { goBack } from '$lib/services/common-services';
	let isSearchingAvailableNetworks = true;
</script>

<Layout title="Manage Network">
	<ListHeading title="Known Networks" />
	<div class="flex flex-col gap-12">
		<div class="flex flex-col gap-4">
			<ListItem
				isLink
				href="/network/manage-network/known/actonate-office-net1"
				title="Actonate Office net1"
			>
				<div class="flex flex-row items-center gap-2">
					<Icons name="blue_checked" height="30px" width="30px" />
					<Icons name="lock" height="30px" width="30px" />
					<Icons name="network" height="30px" width="30px" />
					<Icons name="square_info" height="30px" width="30px" />
				</div>
			</ListItem>
			<ListItem isLink href="/network/manage-network/known/actonate-5g" title="Actonate 5G">
				<div class="flex flex-row items-center gap-2">
					<Icons name="lock" height="30px" width="30px" />
					<Icons name="network" height="30px" width="30px" />
					<Icons name="square_info" height="30px" width="30px" />
				</div>
			</ListItem>
		</div>
		<div>
			<ListHeading title="Available Networks" />
			{#if isSearchingAvailableNetworks}
				<ListItem
					isLink
					href="/network/manage-network/available/actonate-syslink"
					title="Action Linksys"
				>
					<div class="flex flex-row items-center gap-2">
						<Icons name="lock" height="30px" width="30px" />
						<Icons name="network" height="30px" width="30px" />
						<Icons name="square_info" height="30px" width="30px" />
					</div>
				</ListItem>
			{:else}
				<ListItem title="Searching available networks">
					<div class="flex animate-spin flex-row items-center gap-2">
						<Icons name="spinner" height="30px" width="30px" />
					</div>
				</ListItem>
			{/if}
		</div>
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
