/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    "./src/**/*.{html,js}"
  ],
  theme: {
    extend: {
      animation: {
        'cursor': 'blink 1.69s step-start infinite;'
      },
      keyframes: {
        'blink': {
          '0%, 100%': { opacity: 1 },
          '50%': { opacity: 0  },
        }
      }
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}

