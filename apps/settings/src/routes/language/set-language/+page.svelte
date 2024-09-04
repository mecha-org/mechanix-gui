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
		<div class="border-neutral-gray flex flex-row items-center justify-between border-b p-4">
			<div class="items flex flex-row gap-4">
				<p class="text text-lg font-medium">English</p>
				<p class="text text-lg font-medium">English [UK]</p>
			</div>
			<Icons name="blue_checked" height="30px" width="30px" />
		</div>
		<div class="border-neutral-gray flex flex-row items-center justify-between border-b p-4">
			<div class="items flex flex-row gap-4">
				<p class="text text-misty-slate text-lg font-medium">English</p>
				<p class="text text-misty- text-lg font-medium">English [US]</p>
			</div>
			<Icons name="empty_ring" height="30px" width="30px" />
		</div>
		<div class="border-neutral-gray flex flex-row items-center justify-between border-b p-4">
			<div class="items flex flex-row gap-4">
				<p class="text text-misty-slate text-lg font-medium">简体中文</p>
				<p class="text text-misty- text-lg font-medium">Chinese, Simplified</p>
			</div>
			<Icons name="empty_ring" height="30px" width="30px" />
		</div>
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
