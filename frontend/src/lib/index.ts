// place files you want to import through the `$lib` alias in this folder.
import { writable, type Writable } from 'svelte/store';

export let compactToggle = writable(true);

export function isCompact(url: URL, state: boolean) {
	const urlParam = url.searchParams.get('compact');
	if (!urlParam) {
		return state;
	}
	return !(urlParam == 'false');
}
