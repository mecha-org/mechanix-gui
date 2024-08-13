<script lang="ts">
	export let title: string = '';
	export let bluetooth_title: string = '';

	import AstronautXMecha from '$lib/assets/images/wallpapers/astronaut_mecha.png';
</script>

<header>
	{#if title}
		<div class="flex flex-row justify-between p-5 pt-6">
			<h1 class="text-misty-slate text-2xl">{title}</h1>
			<slot name="switch" />
		</div>
	{:else if bluetooth_title.length > 0}
		<div class="flex flex-row p-5 pt-6">
			<h1 class="text-misty-slate text-2xl">
				Confirm this code on<br />
				{' '}
				<strong>
					{bluetooth_title.length > 15
						? `'${bluetooth_title.slice(0, 10)}...${bluetooth_title.substring(bluetooth_title.length - 1)}'`
						: `'${bluetooth_title}'`}
				</strong>
				&nbsp;to connect
			</h1>
		</div>
	{/if}
</header>
<!-- style={ `background-image: url(${AstronautXMecha});`} -->

<main class={`px-5 ${$$slots.footer ? 'pb-20' : 'pb-5'} `}>
	<slot />
</main>

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
