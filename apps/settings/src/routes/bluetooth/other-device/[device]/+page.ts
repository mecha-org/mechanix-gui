import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
export const load: PageLoad = ({ params }) => {
	if (params.device) {
		const modifiedTitle = params.device.split('-').join(' ');
		return { title: modifiedTitle };
	}
	error(404, 'Not found');
};
