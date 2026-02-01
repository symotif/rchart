import { writable } from 'svelte/store';

const ConnectionStatusStore = writable<boolean>(true);

function setConnectionStatus(value: boolean): void {
	ConnectionStatusStore.update(() => value);
}

export { ConnectionStatusStore, setConnectionStatus };
