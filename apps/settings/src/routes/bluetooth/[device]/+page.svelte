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
		console.log('handleForgetDevice ');
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
		<ListItem isLink title={data?.type?.replace('-', ' ')} />
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
			<button
				class=" flex h-[60px] w-[60px] items-center justify-center rounded-lg p-1 text-[#FAFBFC]"
				on:click={handleForgetDevice}
			>
				<Icons name="trash" width="60" height="60" />
			</button>
		</div>
	</footer>
</Layout>
