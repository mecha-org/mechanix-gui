<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { Slider } from '$lib/components/ui/slider';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { goBack } from '$lib/services/common-services';
</script>

<Layout title="Display">
	<ListHeading title="Brightness" />
	<div class="rounded-lg border border-twilight-navy bg-midnight-abyss p-7">
		<Slider value={[33]} max={100} step={1} />
	</div>
	<div class="mt-10">
		<ListItem href="/display/screen-timeoff" isLink title="Screen off timeout">
			<div class="flex flex-row items-center gap-2">
				<p class="text-xl text-misty-slate">30s</p>
				<Icons name="right_arrow" height="30px" width="30px" />
			</div>
		</ListItem>
	</div>
	<div class="mt-10">
		<ListItem isLink title="Auto rotate">
			<Switch />
		</ListItem>
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
				on:click={goBack}
			>
				<Icons name="addition" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
