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
		'focus-visible:ring-ring focus-visible:ring-offset-background peer inline-flex h-[24px] w-[55px] shrink-0 cursor-pointer items-center rounded border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:border-2 data-[state=unchecked]:border-2 data-[state=checked]:border-[#2D8AFF] data-[state=unchecked]:border-[#202431] data-[state=unchecked]:bg-transparent',
		className
	)}
	{...$$restProps}
	on:click
	on:keydown
>
	<slot name="label" />
	<SwitchPrimitive.Thumb
		class={cn(
			'bg-background pointer-events-none mr-2 block h-7 w-7 rounded shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-7 data-[state=unchecked]:translate-x-0 data-[state=checked]:bg-blue-500'
		)}
	/>
</SwitchPrimitive.Root>
