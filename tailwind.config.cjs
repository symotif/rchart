/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors');

module.exports = {
	mode: 'jit',
	//purge: ['./src/**/*.{js,jsx,ts,tsx}'],
	darkMode: 'class',
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				primary: '#202225',
				secondary: '#5865f2',
				gray: colors.trueGray,
				gray: {
					900: '#202225',
					800: '#2f3136',
					700: '#36393f',
					600: '#4f545c',
					400: '#d4d7dc',
					300: '#e3e5e8',
					200: '#ebedef',
					100: '#f2f3f5'
				}
			},
			animation: {
				shimmer: 'shimmer 10s infinite linear'
			},
			keyframes: {
				shimmer: {
					'0%': { backgroundPosition: '-150% 0' },
          '50%': { backgroundPosition: '150% 0' },
					'100%': { backgroundPosition: '-150% 0' }
				}
			},
			backgroundImage: {
				shimmer:
					'linear-gradient(90deg, rgba(255,255,255,0.2) 0%, rgba(255,255,255,0.5) 50%, rgba(255,255,255,0.2) 100%)'
			}
		}
	},
	plugins: []
};
