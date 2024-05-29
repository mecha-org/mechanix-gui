<script lang="ts">
	import { setContext } from 'svelte';
	import Icons from './icons.svelte';
	export let title;
	export let href = '';
	export let isLink = false;
	export let isSelected = false;
	export let isLoading = false;
	export let leftIcon: null | string = null;
</script>

{#if isLink}
	<a
		{href}
		class="flex flex-row items-center justify-between rounded-lg border border-twilight-navy bg-midnight-abyss p-4"
	>
		<h1 class="flex flex-row items-center gap-3 text-lg font-medium text-white">
			{#if leftIcon}
				<Icons name={leftIcon} height="30px" width="30px" />
			{/if}

			{title}
		</h1>
		<slot></slot>
	</a>
{:else if isSelected}
	<div
		class="flex flex-row items-center justify-between rounded-lg border border-twilight-navy bg-midnight-abyss p-4"
	>
		<h1 class="text-lg font-medium text-white">{title}</h1>
		<slot></slot>
	</div>
{:else}
	<div
		class="flex flex-row items-center justify-between rounded-lg border border-twilight-navy bg-midnight-abyss p-4"
	>
		<h1 class="text-lg font-medium text-mid-gray">{title}</h1>
		{#if isLoading}
			<div class="flex animate-spin flex-row items-center gap-2">
				<Icons name="spinner" height="30px" width="30px" />
			</div>
		{/if}
		<slot></slot>
	</div>
{/if}
