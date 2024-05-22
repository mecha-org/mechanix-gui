<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { goBack } from '$lib/services/common-services';
	let batteryModeArr = ['Low', 'Balanced', 'High'];
	let selectedMode = 'Low';
</script>

<Layout title="Performance mode">
	<div class="flex flex-col gap-4">
		<div class="flex flex-col gap-4">
			{#each batteryModeArr as mode, index}
				<button
					on:click={() => {
						selectedMode = mode;
					}}
					><ListItem isSelected={selectedMode == mode} title={mode}>
						{#if mode == selectedMode}
							<Icons name="blue_checked" height="30px" width="30px" />
						{:else}
							<Icons name="empty_ring" height="30px" width="30px" />
						{/if}
					</ListItem></button
				>
			{/each}
		</div>
		{#if selectedMode == 'High'}
			<p class="text-misty-slate">
				Higher performance will use battery faster and increase the temperature of the device
				significantly. Check ambient temperature before proceeding.
			</p>
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
				on:click={goBack}
			>
				<Icons name="tick" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
