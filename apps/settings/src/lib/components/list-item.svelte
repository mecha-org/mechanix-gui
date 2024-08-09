<script lang="ts">
	import Icons from './icons.svelte';
	import type { Events } from './ui/button';

	export let title;
	export let href = '';
	export let isLink = false;
	export let isSelected = false;
	export let leftIcon: null | string = null;
	type $$Events = Events;

</script>

{#if isLink}
	<a
		{href}
		class="flex flex-row items-center justify-between rounded-lg border border-twilight-navy bg-midnight-abyss p-4"
	>
		<h1 class="flex flex-row items-center gap-3 text-lg font-medium text-white text-ellipsis">
			{#if leftIcon}
				<Icons name={leftIcon} height="30px" width="30px" />
			{/if}

			{title}
		</h1>
		<slot></slot>
	</a>
{:else}
	<button
		class="flex flex-row items-center justify-between rounded-lg border border-twilight-navy bg-midnight-abyss p-4 w-full"
		{...$$restProps}
		on:click
		on:keydown
	>
		<h1 class={`text-lg font-medium ${isSelected ? 'text-white' : 'text-mid-gray'}`}>{title}</h1>
		<slot></slot>
	</button>

{/if}
