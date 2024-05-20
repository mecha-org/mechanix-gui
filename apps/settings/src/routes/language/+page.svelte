<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { goBack } from '$lib/services/common-services';
</script>

<Layout title="Language">
	<ListItem title="English" isLink href="/language/set-language">
		<div class="flex flex-row items-center gap-3">
			<p class="text-lg font-medium text-misty-slate">English (UK)</p>
			<Icons name="right_arrow" height="30px" width="30px" />
		</div>
	</ListItem>
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
