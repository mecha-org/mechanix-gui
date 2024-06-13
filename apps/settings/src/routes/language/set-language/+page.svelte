<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import { goBack } from '$lib/services/common-services';
	const submitHandler = () => {
		goBack();
	};
</script>

<Layout title="Select Language">
	<div class="flex flex-col gap-4">
		<div
			class="flex flex-row items-center justify-between rounded-lg border border-twilight-navy bg-midnight-abyss p-4"
		>
			<div class="items flex flex-row gap-4">
				<p class="text text-lg font-medium">English</p>
				<p class="text text-lg font-medium">English [UK]</p>
			</div>
			<Icons name="blue_checked" height="30px" width="30px" />
		</div>
		<div
			class="flex flex-row items-center justify-between rounded-lg border border-twilight-navy bg-midnight-abyss p-4"
		>
			<div class="items flex flex-row gap-4">
				<p class="text text-lg font-medium text-misty-slate">English</p>
				<p class="text text-misty- text-lg font-medium">English [US]</p>
			</div>
			<Icons name="empty_ring" height="30px" width="30px" />
		</div>
		<div
			class="flex flex-row items-center justify-between rounded-lg border border-twilight-navy bg-midnight-abyss p-4"
		>
			<div class="items flex flex-row gap-4">
				<p class="text text-lg font-medium text-misty-slate">简体中文</p>
				<p class="text text-misty- text-lg font-medium">Chinese, Simplified</p>
			</div>
			<Icons name="empty_ring" height="30px" width="30px" />
		</div>
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
					class="bg-ash-gray hover:bg-[#474749]/80 flex h-[48px] w-[48px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
					on:click={submitHandler}
				>
					<Icons name="tick" width="32" height="32" />
				</button>
			</div>
	</footer>
</Layout>
