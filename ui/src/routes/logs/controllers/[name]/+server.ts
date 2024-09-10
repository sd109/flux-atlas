import EventSource from 'eventsource';
import { produce } from 'sveltekit-sse';

export function POST({ params }) {
	const url = `http://localhost:8000/api/controllers/${params.name}/logs`;
	const source = new EventSource(url);

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
