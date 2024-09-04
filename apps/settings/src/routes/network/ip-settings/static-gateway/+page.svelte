<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import { goBack } from '$lib/services/common-services';
	import Input from '$lib/components/ui/input/input.svelte';
	import { Toaster } from 'svelte-french-toast';

	$: gateway = '';

	const submitHandler = () => {
		console.log('submit gateway: ', gateway);
		goBack();
		// goto(`/network/manage-network/connect/${gateway}`);
	};
</script>

<Layout title="Static">
	<div class="mt-6 flex flex-col gap-4">
		<div class="border-neutral-gray border-y-2 py-1">
			<Input placeholder="Gateway" bind:value={gateway} />
		</div>

		<Toaster />
	</div>
	<footer
		slot="footer"
		class="h-full w-full border-t-2 bg-[#05070A73] backdrop-blur-3xl backdrop-filter"
	>
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-1 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="left_arrow" width="60" height="60" />
			</button>
			<button
				class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={submitHandler}
			>
				<Icons name="submit" width="60" height="60" />
			</button>
		</div>
	</footer>
</Layout>
