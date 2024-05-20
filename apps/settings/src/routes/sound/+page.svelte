<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';

	import { Slider } from '$lib/components/ui/slider';
	import { goBack } from '$lib/services/common-services';

	let soundLevel = 50;

	const handleValueCommit = (value: any) => {
		soundLevel = value;
		console.log('JACK JACK handleValueCommit', value);
	};
</script>

<Layout title="Sound">
	<div class="flex flex-col gap-3">
		<div class="flex flex-row items-center justify-between">
			<ListHeading title="Output volume" />
			<p class="text-[22px] font-medium text-[#FFFFFF]">{soundLevel}%</p>
		</div>
		<div class="w-full px-2">
			<Slider onValueChange={handleValueCommit} value={[soundLevel]} max={100} step={1} />
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
