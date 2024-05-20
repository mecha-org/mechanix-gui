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
			class="flex flex-row items-center justify-between rounded-lg border border-twilight-navy bg-midnight-abyss p-4"
		>
			<p class="text-lg text-pale-snow">Automatic Security Update</p>
			<Switch />
		</div>
		<div
			class="mt-4 flex flex-col gap-4 rounded-lg border border-twilight-navy bg-midnight-abyss p-4"
		>
			<div class="w-full">
				<img alt="updates" src={Updates} class="h-[104px] w-full rounded-lg" />
			</div>
			<h1 class="text-lg font-bold text-pale-snow">Mecha V1.1</h1>
			<p class="text-lg font-medium leading-[30px] text-pale-silver">
				This update introduces two more language/ script support in keyboard, bug fixes and security
				updates for the MX device.
				<br />
				<br />
				For information on the security content of Mecha Systems, check out the documentation on mechasystems.com
			</p>
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
