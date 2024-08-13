<script lang="ts">
	import Icons from '$lib/components/icons.svelte';
	import Layout from '$lib/components/layout.svelte';
	import ListItem from '$lib/components/list-item.svelte';
	import { onMount } from 'svelte';
	import { ERROR_LOG, PAGE_LOG, SETTINGS_MODULE_LOG } from '../constants';
	import { consoleLog, LOG_LEVEL } from '$lib/services/common-services';
	import { fetchConnectedWifiInfo, fetchWifiStatus } from '$lib/services/network-services';
	import { connectedNetwork } from '$lib/stores/networkStore';
	import { get_battery_percentage } from '$lib/services/battery-services';
	import { batteryPercentage } from '$lib/stores/batteryStore';

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

	const LOG_PREFIX = PAGE_LOG + SETTINGS_MODULE_LOG;
	const getInitalData = () => {
		consoleLog(LOG_PREFIX + 'getInitalData()::');
		try {
			fetchConnectedWifiInfo();
			get_battery_percentage();
		} catch (error) {
			consoleLog(LOG_PREFIX + 'getInitalData()::' + ERROR_LOG, {
				type: LOG_LEVEL.ERROR,
				data: error
			});
		}
	};

	onMount(() => {
		getInitalData();
	});
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
				<div class="flex flex-row items-center gap-2">
					{#if settings.title == 'Network' && $connectedNetwork.name != undefined}
						<p class="text-misty-slate truncate text-lg">{$connectedNetwork.name}</p>
					{:else if settings.title == 'Battery'}
						<p class="text-misty-slate text-lg">{$batteryPercentage}&percnt;</p>
					{/if}
					<Icons name="right_arrow" height="30px" width="30px" />
				</div>
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
