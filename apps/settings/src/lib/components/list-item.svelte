<script lang="ts">
	import Icons from './icons.svelte';
	export let title;
	export let href = '';
	export let isLink = false;
	export let isSelected = false;
	export let leftIcon: null | string = null;
	export let handler = ()=>{};


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
{:else}
	<button
		class="flex flex-row items-center justify-between rounded-lg border border-twilight-navy bg-midnight-abyss p-4"
		on:click={handler}
	>
		<h1 class={`text-lg font-medium ${isSelected ? 'text-white' : 'text-mid-gray'}`}>{title}</h1>
		<slot></slot>
</button>

{/if}
