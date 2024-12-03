<script>
	let progress = 0;
</script>

<div class="w-full rounded-full bg-gray-200 dark:bg-gray-700">
	<div class="relative h-6 w-full">
		<div
			class="pixelated absolute left-0 top-0 h-full w-full bg-blue-600"
			style="width: {progress}%;"
		></div>
		<div
			class="absolute left-0 top-0 h-full w-full text-center text-xs font-medium leading-6 text-blue-100"
		>
			{progress}%
		</div>
	</div>
</div>

<button
	class="mt-4 rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700"
	on:click={() => (progress = progress < 100 ? progress + 10 : 0)}
>
	{progress === 100 ? 'Reset' : 'Increase Progress'}
</button>

<style>
	.pixelated {
		image-rendering: pixelated;
		image-rendering: crisp-edges;
	}
</style>
