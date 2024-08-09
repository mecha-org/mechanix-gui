<script lang="ts">
	export let title;
	export let isBottomBorderVisible = true;
	export let href = '';
	export let disabled = false;
</script>

<div class="w-full">
	<a {href}>
		<div class="flex w-full justify-between">
			<h1 class={`text-lg font-medium  ${disabled ? 'text-mid-gray' : 'text-white'}`}>{title}</h1>

			<slot></slot>
		</div>
	</a>
	{#if isBottomBorderVisible}<hr class="border-twilight-navy mt-4 border" />{/if}
</div>
