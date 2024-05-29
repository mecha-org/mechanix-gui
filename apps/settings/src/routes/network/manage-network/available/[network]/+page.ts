import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { fetchAvaialbleNetworks, type WirelessInfoResponse } from '$lib/services/network-services';
import { availableNetworksList } from '$lib/stores/networkStore';
export const load: PageLoad = ({ params }) => {


	const {network} = params;

	fetchAvaialbleNetworks();

	let networkDetailList:WirelessInfoResponse[] = []; 

	availableNetworksList.subscribe((value)=>{
		networkDetailList=value;
	});
	
	const selectedNetworkDetails = networkDetailList.find((item)=>item.name == network);

	if(!selectedNetworkDetails){
		return error(404, 'Not found');
	}



	const networkDetail = [
		[
			{
				title: 'Private Wi-Fi Address',
				value: false
			},
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
		return { title:  selectedNetworkDetails?.name, networkDetail: networkDetail };
	}
	error(404, 'Not found');
};
