import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { availableNetworksList, knownNetworksList } from '$lib/stores/networkStore';
import { fetchAvaialbleNetworks, fetchKnownNetworks} from '$lib/services/network-services';
import type { KnownNetworkResponse, WirelessInfoResponse } from '$lib/types/NetworkTypes';


export const load: PageLoad = async ({ params }) => {

	const {network} = params;

	fetchKnownNetworks();
	fetchAvaialbleNetworks();

	let networkList:KnownNetworkResponse[] = []; 
	let networkDetailList:WirelessInfoResponse[] = []; 
	knownNetworksList.subscribe((value)=>{
		networkList=value;
	});

	availableNetworksList.subscribe((value)=>{
		networkDetailList=value;
	});
	const selectedNetwork = networkList.find((item)=>item.network_id == network);
	
	const selectedNetworkDetails = networkDetailList.find((item)=>item.name == selectedNetwork?.ssid);

	if(!selectedNetwork){
		return error(404, 'Not found');
	}

	const displayNetworkDetail = [
		[
			{
				title: 'Network SSID',
				value: selectedNetworkDetails?.name
			},
			{
				title: 'Network ID',
				value: selectedNetwork?.network_id
			},
			{
				title: 'Passphrase',
				value: selectedNetwork?.flags
			},
			{
				title: 'Frequency',
				value: selectedNetworkDetails?.frequency
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
		return { title: selectedNetwork?.ssid , networkDetail: displayNetworkDetail };
	}
	error(404, 'Not found');
};
