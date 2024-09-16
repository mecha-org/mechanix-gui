<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { Slider } from '$lib/components/ui/slider';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { goBack } from '$lib/services/common-services';
	import { getBrightness, setBrightness } from '$lib/services/display-services';
	import { brightnessPercentage } from '$lib/stores/displayStore';
	import { onMount } from 'svelte';

	const getInitalData = async () => {
		await getBrightness();
	};

	const sliderHandler = async (value: number[]) => {
		await setBrightness(value);
	};

	onMount(() => {
		getInitalData();
	});
</script>

<Layout title="Display">
	<ListHeading title="Brightness" />
	<div class="p-4">
		<Slider value={$brightnessPercentage} max={100} step={1} onValueChange={sliderHandler} />
	</div>
	<div class="mt-10">
		<ListItem href="/display/screen-timeoff" isLink title="Screen off timeout">
			<div class="flex flex-row items-center gap-2">
				<p class="text-misty-slate text-xl">30s</p>
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
