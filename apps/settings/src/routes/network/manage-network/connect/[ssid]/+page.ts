import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
export const load: PageLoad = ({ params }) => {

	if (params.ssid) {
		return { title:  params.ssid};
	}
	error(404, 'Not found');
};
