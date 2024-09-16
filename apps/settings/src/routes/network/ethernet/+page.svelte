<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { goBack } from '$lib/services/common-services';
</script>

<Layout title="Ethernet">
	<div slot="switch" class="flex flex-col">
		<Switch />
	</div>
	<ListItem isLink title="IP Settings" href={'/network/ip-settings'}>
		<Icons name="right_arrow" height="30px" width="30px" />
	</ListItem>
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
