<script>
	import Icons from "$lib/components/icons.svelte";
import Layout from "$lib/components/layout.svelte";
	import ListItem from "$lib/components/list-item.svelte";
	import { goBack } from "$lib/services/common-services";
	let timeOutArr = ["10s","20s","30s","40s"]
	let selectedTime = "10s"
</script>
<Layout title="Securtiy timemout">
<div class="flex flex-col gap-4">
   {#each timeOutArr as time,index}
	<button on:click={()=>{selectedTime=time}}><ListItem isSelected={selectedTime==time} title={time}>
	{#if time==selectedTime}
	<Icons name="blue_checked" height="30px" width="30px"/>
	{:else}
	<Icons name="empty_ring" height="30px" width="30px"/>
	{/if}
	</ListItem></button>
   {/each}
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
			class="flex h-[48px] w-[48px]  items-center justify-center rounded-lg bg-ash-gray p-2 text-[#FAFBFC]"
			on:click={goBack}
		>
			<Icons name="tick" width="32" height="32" />
		</button>			
	</div>
</footer>
</Layout>