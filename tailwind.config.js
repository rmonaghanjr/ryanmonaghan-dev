/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./views/**/*.{html,js}",
    ],
    theme: {
        extend: {
            colors: {
                "violet": "#4e4187",
                "blue": "#3083dc",
                "yellow": "#f8ffe5",
                "green": "#7dde92",
                "cyan": "2ebfa5"
            },
            fontFamily: {
                sans: ['Roboto Mono', 'sans-serif'],
                serif: ['Roboto Slab', 'serif']
            }
        },
    },
    plugins: [],
}

