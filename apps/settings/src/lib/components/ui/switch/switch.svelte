<script lang="ts">
	import { Switch as SwitchPrimitive } from 'bits-ui';
	import { cn } from '$lib/utils.js';

	type $$Props = SwitchPrimitive.Props;
	type $$Events = SwitchPrimitive.Events;

	let className: $$Props['class'] = undefined;
	export let checked: $$Props['checked'] = undefined;
	export { className as class };
</script>

<SwitchPrimitive.Root
	bind:checked
	class={cn(
		'focus-visible:ring-ring focus-visible:ring-offset-background data-[state=unchecked]:border-light-gray peer relative inline-flex h-[25px] w-[61px] shrink-0 cursor-pointer items-center border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:border-2 data-[state=unchecked]:border-2 data-[state=checked]:border-[#2D8AFF]',
		className
	)}
	{...$$restProps}
	on:click
	on:keydown
>
	<!-- Text for ON/OFF -->
	<span
		class={cn(
			'absolute text-sm font-bold text-white transition-transform duration-200',
			checked ? 'left-2' : 'right-1'
		)}
	>
		{checked ? 'ON' : 'OFF'}
	</span>
	<SwitchPrimitive.Thumb
		class={cn(
			'bg-background data-[state=unchecked]:bg-light-gray pointer-events-none block h-7 w-7 shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-[32px] data-[state=unchecked]:translate-x-0 data-[state=checked]:bg-blue-500'
		)}
	/>
</SwitchPrimitive.Root>
