const plugin = require('tailwindcss/plugin')
    /** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./templates/**/*.{html.tera,js}"],
    theme: {
        screens: {
            'sm': '640px',
            'md': '768px',
            'lg': '1024px',
            'xl': '1280px',
            '2xl': '1536px',
        },
        darkMode: 'dark'
    },
    plugins: [
        require('@tailwindcss/typography'),
        require('@tailwindcss/forms'),
        require('@tailwindcss/aspect-ratio'),
        require('@tailwindcss/container-queries'),
    ],
}