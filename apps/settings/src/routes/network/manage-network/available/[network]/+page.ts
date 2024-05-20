import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
export const load: PageLoad = ({ params }) => {
	const networkDetail = [
		[
			{
				title: 'Private Wi-Fi Address',
				value: false
			},
			{
				title: 'Wi-Fi Address',
				value: 'A#WELKJEFLK'
			}
		],
		[
			{
				title: 'Network SSID',
				value: 'Action Linksys'
			}
		]
	];
	if (params.network) {
		return { title: params.network, networkDetail: networkDetail };
	}
	error(404, 'Not found');
};
