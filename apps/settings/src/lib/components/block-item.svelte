<script lang="ts">
	export let title;
	export let isBottomBorderVisible: boolean = false;
	export let href = '';
	export let disabled = false;
	export let borderY: boolean = false;
</script>

<a {href} class={`border-neutral-gray w-full  p-4 ${borderY ? 'border-y border-b' : ' '}  `}>
	<div class="flex w-full justify-between">
		<h1 class={`text-lg font-medium  ${disabled ? 'text-mid-gray' : 'text-white'}`}>{title}</h1>
		<slot></slot>
	</div>
</a>
{#if isBottomBorderVisible}<hr class="border-twilight-navy border" />{/if}
