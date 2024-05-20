import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
export const load: PageLoad = ({ params }) => {
	const networkDetail = [
		[
			{
				title: 'Network SSID',
				value: 'Actonate 5G'
			},
			{
				title: 'Network ID',
				value: '2'
			},
			{
				title: 'Passphrase',
				value: 'WPA2'
			},
			{
				title: 'Frequency',
				value: '5GHz'
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
			}
		]
	];
	if (params.network) {
		return { title: params.network, networkDetail: networkDetail };
	}
	error(404, 'Not found');
};
