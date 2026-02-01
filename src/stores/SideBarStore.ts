import { writable } from 'svelte/store';

const SideBarStore = writable<number>(0);

function setTab(value: number): void {
	SideBarStore.update(() => value);
}

export { SideBarStore, setTab };
