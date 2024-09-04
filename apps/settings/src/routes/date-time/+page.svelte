<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { goBack } from '$lib/services/common-services';
</script>

<Layout title="Date & time">
	<div class="flex flex-col gap-4">
		<ListItem title="Enable Automatic Time" isLink>
			<Switch />
		</ListItem>
		<ListItem title="Set time" isLink href="/date-time/set-time">
			<div class="flex flex-row items-center gap-2">
				<p class="text-misty-slate text-lg">UST</p>
				<Icons name="right_arrow" height="30px" width="30px" />
			</div>
		</ListItem>
		<ListItem title="Set date" isLink href="/date-time/set-date">
			<div class="flex flex-row items-center gap-2">
				<p class="text-misty-slate text-lg">January 1,2024</p>
				<Icons name="right_arrow" height="30px" width="30px" />
			</div>
		</ListItem>
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
