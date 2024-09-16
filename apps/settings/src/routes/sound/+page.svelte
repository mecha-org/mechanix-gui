<script lang="ts">
	import SoundItem from '$lib/components/sound-item.svelte';
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';

	import { Slider } from '$lib/components/ui/slider';
	import { goBack } from '$lib/services/common-services';
	import { onMount } from 'svelte';

	import {
		getAllInputDevicesVolume,
		getAllOutputDevicesVolume,
		getInputDevices,
		getOutputDevices,
		setInputDeviceVolume,
		setOutputDeviceVolume,
		updateInputDeviceMute,
		updateOutputDeviceMute
	} from '$lib/services/sound-services';
	import {
		DeviceType,
		inputDevices,
		outputDevices,
		type SoundDevice
	} from '$lib/stores/soundStore';

	const getInitalData = async () => {
		const inputDevicesResponse: any = await getInputDevices();
		const response1 = await getAllInputDevicesVolume(inputDevicesResponse);
		console.log('22. AFTER inputDevices: ', response1);

		const outputDevicesResponse: any = await getOutputDevices();
		const response2 = await getAllOutputDevicesVolume(outputDevicesResponse);
	};

	onMount(() => {
		getInitalData();
	});

	const handleValueCommit = async (value: any, device: string, type: DeviceType) => {
		if (type == DeviceType.INPUT) {
			console.log('slider update for INPUT DEVICE');
			await setInputDeviceVolume(value[0], device);
			if (value[0] == 0) {
				audioClickHandler(DeviceType.INPUT, device, true);
			}
		} else {
			console.log('slider update for OUTPUT DEVICE');
			await setOutputDeviceVolume(value[0], device);
			if (value[0] == 0) {
				audioClickHandler(DeviceType.OUTPUT, device, true);
			}
		}
	};

	const audioClickHandler = (type: DeviceType, device_name: string, is_mute?: boolean) => {
		if (type == DeviceType.INPUT) {
			updateInputDeviceMute(device_name).finally(async () => {
				const updates = $inputDevices.map((device: any) => {
					if (device.name == device_name) device.is_mute = !is_mute;
					if (!is_mute) device.sound_level = [0];
					return device;
				});
				inputDevices.set(updates);

				if (is_mute) await getAllInputDevicesVolume(updates);
			});
		} else {
			updateOutputDeviceMute(device_name).finally(async () => {
				const updates = $outputDevices.map((device: any) => {
					if (device.name == device_name) device.is_mute = !is_mute;
					if (!is_mute) device.sound_level = [0];
					return device;
				});
				outputDevices.set(updates);

				if (is_mute) await getAllOutputDevicesVolume(updates);
			});
		}
	};
</script>

<Layout title="Sound">
	<div class="flex flex-col gap-3">
		<ListHeading title="Output Devices" />
		{#each $outputDevices as outputDevice}
			<SoundItem isBottomBorderVisible={false} title={outputDevice?.description}>
				<div class="flex flex-row">
					<div class="mt-3 flex-1">
						<Slider
							onValueChange={(value) => {
								if (!isNaN(value[0])) {
									handleValueCommit(value, outputDevice?.name, DeviceType.OUTPUT);
								}
							}}
							value={outputDevice?.sound_level || [0]}
							max={100}
							step={1}
						/>
					</div>
					<button
						class="ml-auto px-2"
						on:click={() =>
							audioClickHandler(DeviceType.OUTPUT, outputDevice?.name, outputDevice?.is_mute)}
					>
						{#if outputDevice?.is_mute}
							<Icons name="volume_mute" height="30px" width="30px" />
						{:else}
							<Icons name="volume_unmute" height="30px" width="30px" />
						{/if}
					</button>
				</div>
			</SoundItem>
		{/each}

		<ListHeading title="Input Devices" />
		{#each $inputDevices as inputDevice}
			<SoundItem isBottomBorderVisible={false} title={inputDevice?.description}>
				<div class="flex flex-row">
					<div class="mt-3 flex-1">
						<Slider
							onValueChange={(value) => {
								if (!isNaN(value[0])) {
									handleValueCommit(value, inputDevice?.name, DeviceType.INPUT);
								}
							}}
							value={inputDevice?.sound_level || [0]}
							max={100}
							step={1}
							disabled={inputDevice?.is_mute}
						/>
					</div>

					<button
						class="ml-auto px-2"
						on:click={() =>
							audioClickHandler(DeviceType.INPUT, inputDevice?.name, inputDevice?.is_mute)}
					>
						{#if inputDevice?.is_mute}
							<Icons name="volume_mute" height="30px" width="30px" />
						{:else}
							<Icons name="volume_unmute" height="30px" width="30px" />
						{/if}
					</button>
				</div>
			</SoundItem>
		{/each}
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
