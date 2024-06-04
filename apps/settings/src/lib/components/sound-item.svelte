<script lang="ts">
	export let title;
	export let isBottomBorderVisible = true;
</script>

<div class="w-full rounded-lg border border-twilight-navy bg-midnight-abyss p-4">
	<div class="flex w-full">
		<h1 class="text-xs font-medium text-white">{title}</h1>
	</div>
	<slot></slot>
	{#if isBottomBorderVisible}<hr class="mt-4 border border-twilight-navy" />{/if}
</div>
