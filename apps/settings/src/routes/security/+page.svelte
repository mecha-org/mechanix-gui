<script lang="ts">
	import { goto } from '$app/navigation';
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { goBack } from '$lib/services/common-services';
	import { get_lock_status } from '$lib/services/security-service';
	import {
		disableLockSwitch,
		fetchingLockStatus,
		currentLockStatus
	} from '$lib/stores/securityStore';
	import { onMount } from 'svelte';

	const getInitalData = async () => {
		await get_lock_status();
	};

	const lockStatusHandler = (flag: boolean) => {
		console.log('lockStatusHandler: ', flag);
		// disableLockSwitch - when some data is loading - true  ELSE FALSE
		// true -  change screen - set pin
		// false - stay on same screen
		if (flag) {
			goto('/security/change-pin', { invalidateAll: true });
		}
	};

	const changePinClickHandler = () => {
		goto('/security/change-pin');
	};

	onMount(() => {
		getInitalData();
	});
</script>

<Layout title="Security">
	<div class="flex flex-col gap-4">
		<ListItem title="Enable lock" isLink>
			{#if $fetchingLockStatus}
				<div class="flex animate-spin flex-row items-center gap-2">
					<Icons name="spinner" height="30px" width="30px" />
				</div>
			{:else}
				<Switch
					bind:checked={$currentLockStatus}
					onCheckedChange={lockStatusHandler}
					disabled={$disableLockSwitch}
				/>
			{/if}
		</ListItem>
		<ListItem title="Lock timeout" isLink href="/security/lock-timeout">
			<div class="flex flex-row items-center gap-2">
				<p class="text-lg text-misty-slate">10s</p>
				<Icons name="right_arrow" height="30px" width="30px" />
			</div>
		</ListItem>

		<button
			class="mt-4 flex h-[62px] w-full items-center justify-center rounded-lg bg-[#2F2F39] text-xl font-medium hover:bg-[#2F2F39]/80"
			on:click={changePinClickHandler}
		>
			Change pin
		</button>
	</div>
	<footer slot="footer" class="h-full w-full bg-[#05070A73] backdrop-blur-3xl backdrop-filter">
		<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
			<button
				class="bg-ash-gray flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
				on:click={goBack}
			>
				<Icons name="right_arrow" width="32" height="32" />
			</button>
		</div>
	</footer>
</Layout>
