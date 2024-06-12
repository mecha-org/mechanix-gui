import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { fetchAvaialbleNetworks } from '$lib/services/network-services';
import { availableNetworksList } from '$lib/stores/networkStore';
import type { WirelessInfoResponse } from '$lib/types/NetworkTypes';
export const load: PageLoad = ({ params, url }) => {

	const {network} = params;
	const isConnected = url.searchParams.get('isConnected');

	fetchAvaialbleNetworks();

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
			{
				title: 'Wi-Fi Address',
				value: selectedNetworkDetails?.frequency
			}
		],
		[
			{
				title: 'Network SSID',
				value: selectedNetworkDetails?.name
			}
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
