<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { Button } from '$lib/components/ui/button';

	import { goBack } from '$lib/services/common-services';

	/** @type {import('./$types').PageData} */
	export let data;
</script>

<Layout title={data.title}>
	<div>
		<ListHeading title="Device type" />
		<ListItem isLink title="Mecha MX" />
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
