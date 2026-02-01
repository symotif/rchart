import { writable } from 'svelte/store';

export type ToastType = 'success' | 'error' | 'info' | 'warning';

export interface Toast {
	id: string;
	message: string;
	type: ToastType;
	duration: number;
}

function createToastStore() {
	const { subscribe, update } = writable<Toast[]>([]);

	return {
		subscribe,

		add: (message: string, type: ToastType = 'info', duration: number = 4000) => {
			const id = crypto.randomUUID();
			const toast: Toast = { id, message, type, duration };

			update((toasts) => [...toasts, toast]);

			// Auto-remove after duration
			setTimeout(() => {
				update((toasts) => toasts.filter((t) => t.id !== id));
			}, duration);

			return id;
		},

		success: (message: string, duration?: number) => {
			return ToastStore.add(message, 'success', duration);
		},

		error: (message: string, duration?: number) => {
			return ToastStore.add(message, 'error', duration ?? 6000);
		},

		info: (message: string, duration?: number) => {
			return ToastStore.add(message, 'info', duration);
		},

		warning: (message: string, duration?: number) => {
			return ToastStore.add(message, 'warning', duration ?? 5000);
		},

		remove: (id: string) => {
			update((toasts) => toasts.filter((t) => t.id !== id));
		},

		clear: () => {
			update(() => []);
		}
	};
}

export const ToastStore = createToastStore();
