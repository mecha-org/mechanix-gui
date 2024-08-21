<script lang="ts">
	import BlockItem from '$lib/components/block-item.svelte';
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListBlock from '$lib/components/list-block.svelte';
	import { goBack } from '$lib/services/common-services';
	import Input from '$lib/components/ui/input/input.svelte';
	import { Toaster } from 'svelte-french-toast';
	import { goto } from '$app/navigation';

	function formattitle(title: string) {
		let words = title.split(/[-\s]/);
		for (let i = 0; i < words.length; i++) {
			words[i] = words[i].charAt(0).toUpperCase() + words[i].slice(1);
		}
		return words.join(' ');
	}
	$: network_name = '';

	const goNext = () => {
		goto(`/network/manage-network/connect/${network_name}`);
	};
</script>

<Layout title={`Add a new network`}>
	<div class="mt-6 flex flex-col gap-4">
		<div class="border-neutral-gray border-y-2 py-1">
			<Input placeholder="Network Name" bind:value={network_name} />
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
				on:click={goNext}
			>
				<Icons name="addition" width="60" height="60" />
			</button>
		</div>
	</footer>
</Layout>
