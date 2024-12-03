<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { Progress } from '$lib/components/ui/progress';
	import {
		get_battery_percentage,
		get_selected_performance_mode
	} from '$lib/services/battery-services';
	import { goBack } from '$lib/services/common-services';
	import { batteryPercentage, batteryPerformanceMode } from '$lib/stores/batteryStore';
	import { onMount } from 'svelte';

	const getInitalData = async () => {
		await get_battery_percentage();
		await get_selected_performance_mode();
	};

	onMount(() => {
		getInitalData();
	});
</script>

<Layout title="Battery">
	<div class="flex flex-col gap-4">
		<div>
			<ListHeading title="Battery Percentage" />
			<div class="bg-midnight-abyss border-twilight-navy rounded-lg border p-6">
				<Progress value={$batteryPercentage} />
			</div>
		</div>
		<ListItem title="Battery performace" isLink href="/battery/battery-performance">
			<div class="flex flex-row items-center gap-2">
				<p class="text-misty-slate text-xl">{$batteryPerformanceMode}</p>
				<Icons name="right_arrow" height="30px" width="30px" />
			</div>
		</ListItem>
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
		</div>
	</footer>
</Layout>
