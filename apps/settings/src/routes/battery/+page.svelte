<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListHeading from '$lib/components/list-heading.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { Progress } from '$lib/components/ui/progress';
	import { get_battery_percentage } from '$lib/services/battery-service';
	import { goBack } from '$lib/services/common-services';
	import { batteryPercentage } from '$lib/stores/batteryStore';
	import { onMount } from 'svelte';

	const getInitalData = async () => {
		await get_battery_percentage();
	};

	onMount(()=>{
		getInitalData();
	})
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
			<Icons name="right_arrow" height="30px" width="30px" />
		</ListItem>
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
				on:click={goBack}
			>
				<Icons name="tick" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
