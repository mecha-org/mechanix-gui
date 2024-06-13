<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { goBack } from '$lib/services/common-services';
</script>

<Layout title="Security">
	<div class="flex flex-col gap-4">
		<ListItem title="Enable lock" isLink>
			<Switch />
		</ListItem>
		<ListItem title="Lock timeout" isLink href="/security/lock-timeout">
			<div class="flex flex-row items-center gap-2">
				<p class="text-misty-slate text-lg">10s</p>
				<Icons name="right_arrow" height="30px" width="30px" />
			</div>
		</ListItem>
	</div>
	<footer slot="footer" class="h-full w-full bg-[#05070A73] backdrop-blur-3xl backdrop-filter">
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="bg-ash-gray flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="right_arrow" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
