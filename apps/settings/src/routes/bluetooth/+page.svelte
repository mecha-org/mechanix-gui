<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListBlock from '$lib/components/list-block.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { goBack } from '$lib/services/common-services';
	import { invoke } from '@tauri-apps/api'; 
	import { onMount } from 'svelte';
	import { bluetooth_status } from '$lib/stores';

	let bluetooth_data: any;
	let available_devices: any[];
	$: checked = false;

	const check_bluetooth_data = async () => {
		console.log('check_bluetooth_data called...');
		try {
			let response = await invoke('get_bluetooth_status');
			console.log('response: ', response);
			bluetooth_data = response;

			if (bluetooth_data.status == 1) {
				checked = bluetooth_data.status == 1;
				available_devices = bluetooth_data.available_devices;

				bluetooth_status.set(bluetooth_data.status);
				bluetooth_status.subscribe((value) =>{
					checked = value
				})
			}

		} catch (error) {
			console.error('check_bluetooth_data error : ', error);
		}
	};

	onMount(() => {
		check_bluetooth_data();
	});

	// setInterval(() => {
	// 	check_bluetooth_data();
	// }, 20000);
	
</script>

<Layout title="Bluetooth">
	<ListItem isLink title="Enable bluetooth">
		<Switch bind:checked />
	</ListItem>
	<div class="mt-10">
		<ListHeading title="Available devices" />
		<div class="flex flex-col gap-4">
			<ListItem isLink href="/bluetooth/Ritika's-mecha-compute" title="Ritika's mecha compute">
				<div class="flex flex-row items-center gap-4">
					<Icons name="blue_checked" height="30px" width="30px" />
					<Icons name="right_arrow" height="30px" width="30px" />
				</div>
			</ListItem>
			<ListItem isLink href="/network/dns" title="Ritika's mecha compute 2"
				><Icons name="right_arrow" height="30px" width="30px" /></ListItem
			>
		</div>
	</div>
	<div class="mt-10">
		<ListHeading title="Other devices" />
		<div class="flex flex-col gap-4">
			<ListItem
				isLink
				href="/bluetooth/other-device/ritika's-mecha-compute"
				title="Ritika's mecha compute"
			></ListItem>
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
			<button
				class="flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg bg-ash-gray p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="addition" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
