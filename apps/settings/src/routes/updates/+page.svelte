<script>
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import Updates from '$lib/assets/updates.svg';
	import Layout from '$lib/components/layout.svelte';
	import Icons from '$lib/components/icons.svelte';
	import { goBack } from '$lib/services/common-services';
</script>

<Layout title="Update">
	<div>
		<div
			class="border-neutral-gray flex flex-row items-center justify-between rounded-lg border-b p-4"
		>
			<p class="text-pale-snow text-lg">Auto-update</p>
			<Switch />
		</div>
		<div class="border-neutral-gray mt-4 flex flex-col gap-4 rounded-lg border-b p-4">
			<div class="w-full">
				<img alt="updates" src={Updates} class="h-[104px] w-full rounded-lg" />
			</div>
			<h1 class="text-pale-snow text-lg font-bold">Mecha V1.1</h1>
			<p class="text-pale-silver text-lg font-medium leading-[30px]">
				This update introduces two more language/ script support in keyboard, bug fixes and security
				updates for the MX device.
				<br />
				<br />
				For information on the security content of Mecha Systems, check out the documentation on mechasystems.com
			</p>
		</div>
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
