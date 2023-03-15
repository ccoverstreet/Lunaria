import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	server: {
		proxy: {
			"/api": {
				target: "http://localhost:9090",
				changeOrigin: true,
				//rewrite: (path) => path.replace(/^\/api/, '')
			}
		}
	},
	plugins: [sveltekit()]
});
