<script lang="ts">
	import TopPanel from './top-panel.svelte';
	export let title: string = '';

	import AstronautXMecha from '$lib/assets/images/wallpapers/astronaut_mecha.png';

</script>

<!-- <header class="z-10 flex h-[80px] flex-col bg-[#05070A]"> -->
<header>
	<div class="px-5 py-2">
		<TopPanel />
	</div>
	<div class="p-5 pt-2">
		<h1 class="text-misty-slate text-2xl">{title}</h1>
	</div>
</header>
<!-- style={ `background-image: url(${AstronautXMecha});`} -->

<main class={`px-5 ${$$slots.footer ? 'pb-20' : 'pb-5'} `}
><slot /></main>

{#if $$slots.footer}
	<footer
		style="height: 80px; position: fixed; bottom: 0; width: 100%; text-align: center; z-index: 10;"
	>
		<slot name="footer" />
	</footer>
{/if}

<style>
	header {
		margin: 0;
		padding: 0;
	}

	footer {
		text-align: center;
	}
</style>
