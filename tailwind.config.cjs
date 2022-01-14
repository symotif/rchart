const colors = require('tailwindcss/colors')

module.exports = {
	purge: ['./src/**/*.svelte', './src/**/*.css'], // do we need this?
	content: ['./src/**/*.{html,js,svelte,ts}'],
	mode: 'jit',
	darkMode: false,

	theme: {
		extend: {
			colors: {
				primary: '#202225',
				secondary: '#5865f2',
			},
		},
	},

	variants: {
		extend: {}
	},

	plugins: [require('daisyui')]
};