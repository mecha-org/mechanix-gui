<script lang="ts">
	import AstronautXMecha from '$lib/assets/images/wallpapers/astronaut_mecha.png';
	import IllustrationXMecha from '$lib/assets/images/wallpapers/illustration_mecha.png';
	import CometXMecha from '$lib/assets/images/wallpapers/comet_mecha.png';
	import Layout from '$lib/components/layout.svelte';
	import Icons from '$lib/components/icons.svelte';
	import { goBack } from '$lib/services/common-services';
	import ListHeading from '$lib/components/list-heading.svelte';

	const modes = {
		choose: 'CHOOSE',
		apply: 'APPLY'
	};

	const systemWallpapers = [
		{ key: 'default', name: 'Default', src: IllustrationXMecha },
		{ key: 'space', name: 'Space', src: AstronautXMecha },
		{ key: 'comet', name: 'Comet in sky', src: CometXMecha }
	];

	type modeStates = 'CHOOSE' | 'APPLY';

	let currScreen: modeStates = modes.choose as modeStates;
	let selectedImage = systemWallpapers[0];

	const selectImage = (image: { key: string; name: string; src: string }) => {
		selectedImage = image;
	};

	const applyTheme = (value: string) => {
		currScreen = value as modeStates;
	};
</script>

{#if currScreen === modes.choose}
	<Layout title="Appearance">
		<div class="flex flex-col gap-3">
			<ListHeading title="Choose wallpaper" />
			<ul class="flex flex-col gap-3">
				<div class="flex flex-row items-center gap-5">
					{#each systemWallpapers as image, index}
						<li id={`${index}`}>
							<div
								on:click={() => selectImage(image)}
								on:keydown={() => selectImage(image)}
								class="flex cursor-pointer flex-col gap-3"
								role="cell"
								tabindex={index}
							>
								<img
									alt={image.name}
									src={image.src}
									class={`rounded-xl ${selectedImage.key === image.key ? 'h-[120px] w-[120px] border-2 border-primary' : 'h-[120px] w-[120px] border-2 border-[#2A2A2C]'}  transition-all duration-100 ease-in-out`}
								/>
								<p
									class={`text-base font-medium ${selectedImage.key === image.key ? 'text-[#DBDDE1]' : 'text-[#858586]'}`}
								>
									{image.name}
								</p>
							</div>
						</li>
					{/each}
				</div>
			</ul>
		</div>
		<footer slot="footer" class="h-full w-full bg-[#05070A73] backdrop-blur-3xl backdrop-filter">
			<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
				<button
					class="flex h-[48px] w-[48px] rotate-180 items-center justify-center rounded-md bg-[#15171D] p-2 text-[#FAFBFC]"
					on:click={goBack}
				>
					<Icons name="right_arrow" width="32" height="32" />
				</button>
				<button
					class="flex h-[48px] w-[48px] items-center justify-center rounded-md bg-[#15171D] p-2 text-primary"
					on:click={() => applyTheme(modes.apply)}
				>
					<Icons name="export" width="32" height="32" />
				</button>
			</div>
		</footer>
	</Layout>
{:else}
	<div class="m-0 max-h-screen overflow-hidden p-0">
		<header
			class="absolute top-0 z-10 flex h-[80px] w-full items-center bg-[#05070A73] backdrop-blur-3xl backdrop-filter"
		>
			<div class="px-5 py-7">
				<h1 class="text-[26px] text-xl font-normal text-[#B7BBC8]">
					Apply theme "{selectedImage.name}"
				</h1>
			</div>
		</header>
		<div class="flex flex-col gap-3">
			<img
				alt={selectedImage.name}
				src={selectedImage.src}
				class={`h-full w-full rounded-xl object-cover`}
			/>
		</div>
	</div>
{/if}
