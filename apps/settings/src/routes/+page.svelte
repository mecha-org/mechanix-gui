<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import TopPanel from '$lib/components/top-panel.svelte';

	let settingsListArr1 = [
		{
			title: 'Network',
			icon: 'network_box',
			link: '/network'
		},
		{
			title: 'Bluetooth',
			icon: 'bluetooth_box',
			link: '/bluetooth'
		},
		{
			title: 'Display',
			icon: 'brush_box',
			link: '/display'
		},
		{
			title: 'Appearance',
			icon: 'appearance_box',
			link: '/appearance'
		},
		{
			title: 'Battery',
			icon: 'battery_box',
			link: '/battery'
		},
		{
			title: 'Sound',
			icon: 'sound_box',
			link: '/sound'
		},
		{
			// title: 'Security',
			title: 'Lock',
			icon: 'security_box',
			link: '/security'
		}
	];

	let settingsListArr2 = [
		{
			title: 'Date & Time',
			icon: 'time_box',
			link: '/date-time'
		},
		{
			title: 'Language',
			icon: 'language_box',
			link: '/language'
		},
		{
			title: 'Updates',
			icon: 'updates_box',
			link: '/updates'
		},
		{
			title: 'About',
			icon: 'about_box',
			link: '/about'
		}
	];
</script>

<Layout title="Settings">
	<div class="flex flex-col gap-x-3">
		{#each settingsListArr1 as settings, index}
			<ListItem
				title={settings.title}
				isLink
				href={settings.link}
				leftIcon={settings.icon}
				borderTop={index != 0}
			>
				<Icons name="right_arrow" height="30px" width="30px" />
			</ListItem>
		{/each}
	</div>
	<div class="my-5">
		<hr />
	</div>
	<div class="flex flex-col gap-x-3">
		{#each settingsListArr2 as settings}
			<ListItem title={settings.title} isLink href={settings.link} leftIcon={settings.icon}>
				<Icons name="right_arrow" height="30px" width="30px" />
			</ListItem>
		{/each}
	</div>
</Layout>
