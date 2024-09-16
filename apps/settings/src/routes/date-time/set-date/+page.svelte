<script>
	import Icons from "$lib/components/icons.svelte";
	import Layout from "$lib/components/layout.svelte";
	import ListHeading from "$lib/components/list-heading.svelte";
	import Input from "$lib/components/ui/input/input.svelte";
	import { goBack } from "$lib/services/common-services";

</script>
<Layout title="Set date">
    <div class="w-full grid grid-cols-3 gap-4">
        <div><Input/></div>
        <div><Input/></div>
        <div><Input/></div>
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