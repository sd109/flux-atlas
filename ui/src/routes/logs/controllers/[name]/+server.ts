import EventSource from 'eventsource';
import { produce } from 'sveltekit-sse';
import { API_BASE_URL } from '../../../../config.js';

export function POST({ params }) {
	const url = new URL(`controllers/${params.name}/logs`, API_BASE_URL);
	const source = new EventSource(url.toString());

	source.onopen = () => console.log('Connected to event source:', params.name);
	source.onerror = () => {
		console.log('Closing event source:', params.name);
		source.close();
	};

	return produce(async function start({ emit }) {
		source.onmessage = (event) => {
			const { error } = emit('message', event.data);
			if (error) {
				console.log('Connected to event source:', params.name);
				source.close();
				return;
			}
		};
	});
}
