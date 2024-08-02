<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { Button } from '$lib/components/ui/button';
	import Input from '$lib/components/ui/input/input.svelte';
	import { addBluetoothDevice } from '$lib/services/bluetooth-services';

	import { goBack } from '$lib/services/common-services';
	import { invoke } from '@tauri-apps/api';

	/** @type {import('./$types').PageData} */
	export let data;

	let pin = '';
	const handleAddDevice =  () => {
		console.log("handleAddDevice ");
		const { address } = data;
		try {
			 addBluetoothDevice(address);
			 goBack();
		} catch (error) {
			console.error('page::bluetooth::handleAddDevice::error:::: ', error);
		}
	};
</script>

<Layout title={`Pair with ${data.title}`}>
	<div>
		<ListHeading title={`Enter code shared by the device here`} />
		<Input placeholder="Type here" bind:value={pin} maxlength={6} />
		<!-- <Input placeholder="Type here" bind:value={data?.details?.address} /> -->
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
				class="bg-ash-gray flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={handleAddDevice}
			>
				<Icons name="addition" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
