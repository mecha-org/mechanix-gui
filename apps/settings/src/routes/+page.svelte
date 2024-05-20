<script>
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	let settingsListArr = [
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
			title: 'Security',
			icon: 'security_box',
			link: '/security'
		},
		{
			title: 'Date & Time',
			icon: 'date_box',
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
	<div class="flex flex-col gap-3">
		{#each settingsListArr as settings}
			<ListItem title={settings.title} isLink href={settings.link} leftIcon={settings.icon}>
				<Icons name="right_arrow" height="30px" width="30px" />
			</ListItem>
		{/each}
	</div>
</Layout>
