<script lang="ts">
	import Icons from './icons.svelte';
	import type { Events } from './ui/button';

	export let title;
	export let href = '';
	export let isLink = false;
	export let isSelected = false;
	export let leftIcon: null | string = null;
	export let borderTop: boolean = false;

	type $$Events = Events;
</script>

{#if isLink}
	<a
		{href}
		class={`border-neutral-gray flex flex-row items-center justify-between ${borderTop ? 'border-t' : 'border-b'} p-4`}
	>
		<h1 class="flex flex-row items-center gap-3 text-ellipsis text-lg font-medium text-white">
			{#if leftIcon}
				<Icons name={leftIcon} height="30px" width="30px" />
			{/if}

			{title}
		</h1>
		<slot></slot>
	</a>
{:else}
	<div class={`border-neutral-gray flex flex-col ${borderTop ? 'border-t' : 'border-b'}`}>
		<button
			class={`flex w-full flex-row items-center justify-between  p-4 `}
			{...$$restProps}
			on:click
			on:keydown
		>
			<h1 class={`text-lg font-medium ${isSelected ? 'text-white' : 'text-mid-gray'}`}>{title}</h1>
			<slot></slot>
		</button>
		<slot name="content" class="p-4" />
	</div>
{/if}
