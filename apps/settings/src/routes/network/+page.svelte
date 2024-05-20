<script>
	import BlockItem from '$lib/components/block-item.svelte';
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListBlock from '$lib/components/list-block.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { goBack } from '$lib/services/common-services';
</script>

<Layout title="Network">
	<div class="flex flex-col gap-12">
		<div class="flex flex-col gap-4">
			<ListBlock>
				<BlockItem title="Enable Wireless">
					<Switch />
				</BlockItem>
				<BlockItem isBottomBorderVisible={false} title="Actonate Office net1">
					<div class="flex flex-row items-center gap-4">
						<Icons height="30px" width="30px" name="blue_checked" />
						<Icons height="30px" width="30px" name="right_arrow" />
					</div>
				</BlockItem>
			</ListBlock>
			<ListItem isLink href="/network/manage-network" title="Manage Networks"
				><Icons name="right_arrow" height="30px" width="30px" /></ListItem
			>
			<ListItem isLink href="/network/ip-settings" title="IP Settings"
				><Icons name="right_arrow" height="30px" width="30px" /></ListItem
			>
		</div>
		<div>
			<ListHeading title="Others" />
			<div class="flex flex-col gap-4">
				<ListItem isLink href="/network/ethernet" title="Ethernet"
					><Icons name="right_arrow" height="30px" width="30px" /></ListItem
				>
				<ListItem isLink href="/network/dns" title="DNS"
					><Icons name="right_arrow" height="30px" width="30px" /></ListItem
				>
			</div>
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
		</div>
	</footer>
</Layout>
