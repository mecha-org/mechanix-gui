<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { get_all_performance_mode, set_performance_mode } from '$lib/services/battery-services';
	import { goBack } from '$lib/services/common-services';
	import { batteryPerformanceMode, batteryPerformanceOptions } from '$lib/stores/batteryStore';
	import { onMount } from 'svelte';

	let selectedMode = $batteryPerformanceMode;

	const getInitalData = async () => {
		await get_all_performance_mode();
		selectedMode = $batteryPerformanceMode;
	};

	const modeHandler = (mode: string) => async () => {
		selectedMode = mode;
	};

	const submitHandler = async () => {
		await set_performance_mode(selectedMode);
		goBack();
	};

	onMount(() => {
		getInitalData();
	});
</script>

<Layout title="Performance mode">
	<div class="flex flex-col gap-4">
		<div class="flex flex-col gap-4">
			{#each $batteryPerformanceOptions as mode, index}
				<button on:click={modeHandler(mode)}>
					<ListItem isSelected={selectedMode == mode} title={mode}>
						{#if mode == selectedMode}
							<Icons name="blue_checked" height="30px" width="30px" />
						{:else}
							<Icons name="empty_ring" height="30px" width="30px" />
						{/if}
					</ListItem>
				</button>
			{/each}
		</div>
		{#if selectedMode == 'High'}
			<p class="text-misty-slate">
				Higher performance will use battery faster and increase the temperature of the device
				significantly. Check ambient temperature before proceeding.
			</p>
		{/if}
	</div>
	<footer
		slot="footer"
		class="h-full w-full border-t-2 bg-[#05070A73] backdrop-blur-3xl backdrop-filter"
	>
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-1 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="left_arrow" width="60" height="60" />
			</button>
			<button
				class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={submitHandler}
			>
				<Icons name="submit" width="60" height="60" />
			</button>
		</div>
	</footer>
</Layout>
