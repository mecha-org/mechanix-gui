<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import { goBack } from '$lib/services/common-services';
	import Input from '$lib/components/ui/input/input.svelte';
	let pin: string = '';

</script>

<Layout title="Enter Security Pin">
	<div class="flex flex-col gap-4">
		<Input placeholder="Enter Pin" bind:value={pin} />
	</div>
	<footer slot="footer" class="h-full w-full bg-[#05070A73] backdrop-blur-3xl backdrop-filter">
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="bg-ash-gray flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="right_arrow" width="32" height="32" />
			</button>
			<button
				class="bg-ash-gray flex h-[48px] w-[48px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="tick" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
