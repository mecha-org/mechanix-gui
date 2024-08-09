import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
export const load: PageLoad = ({ params, url }) => {
	if (params.device) {
		const modifiedTitle = params.device.split('-').join(' ');
		return { 
			title: modifiedTitle,
			address: url.searchParams.get('address'),
			type: url.searchParams.get('type')
		 };
	}
	error(404, 'Not found');
};
