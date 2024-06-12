<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { LOG_LEVEL, consoleLog, goBack } from '$lib/services/common-services';
	import { invoke } from '@tauri-apps/api/tauri';

	import type { PageData } from '../../available/[network]/$types';
	import { fetchKnownNetworks, removeWifi } from '$lib/services/network-services';
	import { ERROR_LOG, NETWORK_MODULE_LOG, PAGE_LOG } from '../../../../../constants';

	const LOG_PREFIX = PAGE_LOG + NETWORK_MODULE_LOG + 'manage-network::connet::';

	export let data: PageData;
	let showError : boolean = false;
	let error_message : string = "";

	$: password = '';
	const connectToNetwork = async () => {
		consoleLog(LOG_PREFIX + 'connectToNetwork()::');
		try {
			const response: boolean = await invoke('connect_to_network', {
				ssid: data.title,
				password: password
			});

			await fetchKnownNetworks();
			goBack();
		} catch (error: any) {
			console.log('=======> error: ', error);
			const startIndex = error.indexOf('message:') + 'message:'.length;
			const endIndex = error.length;
			error_message = error.substring(startIndex, endIndex).replace(")","").trim();
			console.log("=======> error_message:", error_message);
			showError = true;
			
			consoleLog(LOG_PREFIX + 'connectToNetwork()::' + ERROR_LOG, {
				type: LOG_LEVEL.ERROR,
				data: error
			});

			setTimeout(() => {showError=false}, 3000);
		}
	};
</script>

<Layout title={data.title}>
	<div class="flex flex-col gap-4">
		<div>
			<ListHeading title="Password" />
			<Input placeholder="Password" bind:value={password} />
		</div>

		{#if showError}
			<div class="animate-pulse capitalize text-gray-300 text-lg inline-flex justify-center">
				{error_message}
			</div>
		{/if}
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
				class="bg-ash-gray flex h-[48px] w-[48px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={connectToNetwork}
			>
				<Icons name="tick" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
