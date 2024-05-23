<script lang="ts">
	export let title:string = "";
</script>

<header class="z-10 flex h-[80px] items-center bg-[#05070A]">
	<div class="p-5">
		<h1 class="text-xl font-bold text-misty-slate">{title}</h1>
	</div>
</header>

<main class={`px-5 ${$$slots.footer ? 'pb-20' : 'pb-5'} `}><slot /></main>

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
