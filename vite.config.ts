import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import fs from 'fs';

const viteTauriSetup = fs.readFileSync('./vite-tauri-setup.json', 'utf-8');
const tauriConfig = JSON.parse(viteTauriSetup);
const port = parseInt(tauriConfig.build.devPath.split(':').pop() || '5173');

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		port,
		strictPort: true
	}
});
