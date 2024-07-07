export function randomString(length: number) {
	const characters = 'abcdefghijklmnopqrstuvwxyz';
	let result = '';
	for (let i = 0; i < length; ++i) {
		result += characters[Math.floor(Math.random() * characters.length)];
	}
	return result;
}
