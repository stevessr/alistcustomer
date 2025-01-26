/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './index.html',
    './src/**/*.{vue,js,ts,jsx,tsx}'
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#3b82f6',
          dark: '#2563eb'
        },
        secondary: {
          DEFAULT: '#64748b',
          dark: '#475569'
        }
      },
      spacing: {
        18: '4.5rem'
      },
      borderRadius: {
        xl: '1rem'
      }
    },
  },
  plugins: [],
}
