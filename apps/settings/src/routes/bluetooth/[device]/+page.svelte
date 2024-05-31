<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { Button } from '$lib/components/ui/button';
	import { forgetBluetoothDevice } from '$lib/services/bluetooth-services';

	import { goBack } from '$lib/services/common-services';
	import { invoke } from '@tauri-apps/api';

	/** @type {import('./$types').PageData} */
	export let data;

	const handleForgetDevice = () => {
		console.log("handleForgetDevice ");
		const { address } = data;
		try {
			forgetBluetoothDevice(address);
			goBack();
		} catch (error) {
			console.error('page::bluetooth::handleForgetDevice::error:::: ', error);
		}
	};
</script>

<Layout title={data.title}>
	<div>
		<ListHeading title="Device type" />
		<ListItem isLink title={data?.type?.replace("-", " ")} />
	</div>

	<button
		class="mt-10 flex h-[62px] w-full items-center justify-center rounded-lg bg-[#2F2F39] text-xl font-medium text-[#F33742] hover:bg-[#2F2F39]/80"
		on:click={handleForgetDevice}
	>
		Forget this Device
	</button>

	<footer slot="footer" class="h-full w-full bg-[#05070A73] backdrop-blur-3xl backdrop-filter">
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="bg-ash-gray flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="right_arrow" width="32" height="32" />
			</button>
			<button
				class="bg-ash-gray flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="addition" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
