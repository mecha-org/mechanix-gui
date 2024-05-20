<script lang="ts">
	export let title;
	export let isBottomBorderVisible = true;
</script>

<div class="w-full">
	<div class="flex w-full justify-between">
		<h1 class="text-lg font-medium text-white">{title}</h1>
		<slot></slot>
	</div>
	{#if isBottomBorderVisible}<hr class="mt-4 border border-twilight-navy" />{/if}
</div>
