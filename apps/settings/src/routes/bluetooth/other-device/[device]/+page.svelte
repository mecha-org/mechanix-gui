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

	const { code, address } = data;

	let pin = '';
	const handleAddDevice = () => {
		console.log('handleAddDevice: ', data);
		// handle code received or entered case
		try {
			addBluetoothDevice(address);
			goBack();
		} catch (error) {
			console.error('page::bluetooth::handleAddDevice::error:::: ', error);
		}
	};
</script>

<Layout title={!code ? `Pair with ${data.title}` : ``} bluetooth_title={data.title}>
	<div class="border-neutral-gray mt-3 border-y-2 py-3">
		{#if !code}
			<Input placeholder="Enter code on the device" bind:value={pin} maxlength={6} />
		{:else}
			<div class="py-4 text-center text-3xl font-bold tracking-[.60em]">
				{code}
			</div>
		{/if}
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

			{#if !code}
				<button
					class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-1 text-[#FAFBFC]"
					on:click={handleAddDevice}
				>
					<Icons name="addition" width="60" height="60" />
				</button>
			{:else}
				<button
					class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-1 text-[#FAFBFC]"
					on:click={handleAddDevice}
				>
					<Icons name="submit" width="60" height="60" />
				</button>
			{/if}
		</div>
	</footer>
</Layout>
