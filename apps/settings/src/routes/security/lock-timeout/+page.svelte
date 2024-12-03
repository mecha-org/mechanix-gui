<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { goBack } from '$lib/services/common-services';
	let timeOutArr = ['10s', '20s', '30s', '40s'];
	let selectedTime = '10s';
</script>

<Layout title="Securtiy timemout">
	<div class="flex flex-col gap-4">
		{#each timeOutArr as time, index}
			<button
				on:click={() => {
					selectedTime = time;
				}}
			>
				<ListItem isSelected={selectedTime == time} title={time}>
					{#if time == selectedTime}
						<Icons name="blue_checked" height="30px" width="30px" />
					{:else}
						<Icons name="empty_ring" height="30px" width="30px" />
					{/if}
				</ListItem>
			</button>
		{/each}
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
				on:click={goBack}
			>
				<Icons name="submit" width="60" height="60" />
			</button>
		</div>
	</footer>
</Layout>
