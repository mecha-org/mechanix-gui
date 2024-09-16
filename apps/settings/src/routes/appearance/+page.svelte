<script lang="ts">
	import AstronautXMecha from '$lib/assets/images/wallpapers/astronaut_mecha.png';
	import IllustrationXMecha from '$lib/assets/images/wallpapers/illustration_mecha.png';
	import CometXMecha from '$lib/assets/images/wallpapers/comet_mecha.png';
	import Layout from '$lib/components/layout.svelte';
	import Icons from '$lib/components/icons.svelte';
	import { goBack } from '$lib/services/common-services';

	const modes = {
		choose: 'CHOOSE',
		apply: 'APPLY'
	};

	const systemWallpapers = [
		{ key: 'default', name: 'Default', src: IllustrationXMecha },
		{ key: 'space', name: 'Spacecraft', src: AstronautXMecha },
		{ key: 'comet', name: 'Comet', src: CometXMecha }
	];

	type modeStates = 'CHOOSE' | 'APPLY';

	let currentScreen: modeStates = modes.choose as modeStates;
	let selectedImage = systemWallpapers[0];

	const selectImage = (image: { key: string; name: string; src: string }) => {
		selectedImage = image;
		currentScreen = modes.apply as modeStates;
	};

	// const selectTheme = (value: string) => {
	// 	currentScreen = value as modeStates;
	// };

	const submitHandler = () => {};

	const goBackToChooseWallpaper = () => {
		currentScreen = modes.choose as modeStates;
	};
</script>

{#if currentScreen === modes.choose}
	<Layout title="Appearance">
		<div class="flex flex-col gap-3">
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
									class={`rounded${selectedImage.key === image.key ? 'h-[120px] w-[120px] border-2 border-blue-500' : 'h-[120px] w-[120px] border-2 border-[#2A2A2C]'}  transition-all duration-100 ease-in-out`}
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
			<div
				class="border-silver-gray flex h-full w-full flex-row items-center justify-between border-t-2 px-4 py-3"
			>
				<button
					class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-1 text-[#FAFBFC]"
					on:click={goBack}
				>
					<Icons name="left_arrow" width="60" height="60" />
				</button>
			</div>
		</footer>
	</Layout>
{:else}
	<Layout title="Apply wallpaper ">
		<div class="flex flex-col gap-3">
			<img
				alt={selectedImage.name}
				src={selectedImage.src}
				class={`h-full w-full rounded-xl object-cover`}
			/>
		</div>
		<footer
			slot="footer"
			class="h-full w-full border-t-2 bg-[#05070A73] backdrop-blur-3xl backdrop-filter"
		>
			<div class="flex h-full w-full flex-row items-center justify-between px-4 py-3">
				<button
					class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-1 text-[#FAFBFC]"
					on:click={goBackToChooseWallpaper}
				>
					<Icons name="left_arrow" width="60" height="60" />
				</button>
				<button
					class="flex h-[60px] w-[60px] items-center justify-center rounded-lg p-2 text-[#FAFBFC]"
					on:click={submitHandler}
				>
					<Icons name="submit" width="60" height="60" />
				</button>
			</div>
		</footer>
	</Layout>
{/if}
