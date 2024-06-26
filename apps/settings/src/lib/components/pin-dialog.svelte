<script lang="ts">
	import Input from '$lib/components/ui/input/input.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import Icons from './icons.svelte';
	import { createEventDispatcher } from 'svelte';

	export let title: string = '';
	export let pinValue: string = '';
	export let showError: boolean = false;
	export let errorMessage: string = '';
	type KeysType = {
		value: string;
		icon?: string;
	};
	let keysArray: KeysType[] = [
		{ value: '1' },
		{ value: '2' },
		{ value: '3' },
		{ value: '4' },
		{ value: '5' },
		{ value: '6' },
		{ value: '7' },
		{ value: '8' },
		{ value: 'cancel', icon: 'cancel' as string }, // back
		{ value: '9' },
		{ value: '0' },
		{ value: 'backspace', icon: 'backspace' as string } // backspace - erase
	];

	const dispatch = createEventDispatcher();

</script>

<style>
	.body{
		background-color: bisque;
	}
</style>

<div class="flex gap-4 bg-white">
	<Dialog.Root
		open={true}
		onOutsideClick={(e) => {
			e.preventDefault();
		}}
	>
		<Dialog.Content class="h-[70%] w-[70%] rounded-lg border-0 bg-[#15171D;]">
			<Dialog.Header class="">
				<Dialog.Title class="flex justify-center">
					{title}
				</Dialog.Title>
			</Dialog.Header>

			<Dialog.Description class="h-full text-white">
				{#if showError}
					<div
						class="flex animate-pulse items-center justify-center text-lg normal-case text-gray-400"
					>
						{errorMessage}
					</div>
				{/if}
				<div class="flex flex-1 items-center justify-center">
					<Input
						class="text-center text-xl"
						placeholder="Enter pin"
						type="password"
						bind:value={pinValue}
					/>
				</div>
				<div class="flex flex-1 items-center justify-center">
					<div class="grid grid-cols-4 gap-4 p-4">
						{#each keysArray as key (key?.value)}
							<button
								class="rounded-md bg-[#2C2F36] px-2.5 py-1.5 text-2xl font-bold text-white"
								on:click={() => {
									dispatch('click', key.value);
								}}
							>
								{#if key.icon && (key.value == 'cancel' || key.value == 'backspace')}
									<Icons name={key.icon} height="30px" width="30px" />
								{:else}
									{key.value}
								{/if}
							</button>
						{/each}
					</div>
				</div>
			</Dialog.Description>
		</Dialog.Content>
	</Dialog.Root>
</div>
