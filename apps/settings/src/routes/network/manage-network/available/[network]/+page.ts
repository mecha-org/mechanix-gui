import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { fetchAvaialbleNetworks } from '$lib/services/network-services';
import { availableNetworksList } from '$lib/stores/networkStore';
import type { WirelessInfoResponse } from '$lib/types/NetworkTypes';
export const load: PageLoad = async ({ params, url }) => {

	const {network} = params;
	const isConnected = url.searchParams.get('isConnected');

	await fetchAvaialbleNetworks();

	let networkDetailList:WirelessInfoResponse[] = []; 

	availableNetworksList.subscribe((value)=>{
		networkDetailList=value;
	});

	const selectedNetworkDetails = networkDetailList?.find((item)=>item.name == network);

	if(!selectedNetworkDetails){
		console.log("404 ====> IF selectedNetworkDetails NOT FOUND! ");
		return error(404, 'Not found');
	}

	const networkDetail = [
		[
			// // NOTE: ARCHIVED 
			// {
			// 	title: 'Private Wi-Fi Address',
			// 	value: false
			// },
			// {
			// 	title: 'Network ID',
			// 	value: selectedNetworkDetails?.network_id
			// },
			// {
			// 	title: 'Passphrase',
			// 	value: selectedNetworkDetails?.flags
			// },
			{
				title: 'Network SSID',
				value: selectedNetworkDetails?.name
			},
			{
				title: 'Frequency',
				value: selectedNetworkDetails?.frequency
			},
			{
				title: 'Security',
				value: selectedNetworkDetails?.security
			},
			{
				title: 'Encryption',
				value: selectedNetworkDetails?.encryption
			}
		],
		[
			{
				title: 'IP Address',
				value: '192.160.12.1'
			},
			{
				title: 'Subnet Mask',
				value: '255.255.255.0'
			},
			{
				title: 'Gateway',
				value: '192.160.0.1'
			},
			{
				title: 'MAC Address',
				value: selectedNetworkDetails?.mac
			},
		]
	];

	if (params.network) {
		return { 
			title:  selectedNetworkDetails?.name, 
			networkDetail: networkDetail,
			isConnected: isConnected
		}
	}
	error(404, 'Not found');
};
