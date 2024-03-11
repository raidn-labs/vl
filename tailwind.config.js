/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/components/**/*.vue",
    "./src/**/*.vue",
  ],
  theme: {
    extend: {
      fontSize: {
        xs: ['1.125rem', { lineHeight: '1.75rem' }], // 18px
        sm: ['1.25rem', { lineHeight: '1.75rem' }], // 20px
        base: ['1.5rem', { lineHeight: '2rem' }], // 24px
        md: ['1.875rem', { lineHeight: '2.25rem' }], // 30px
        lg: ['2.25rem', { lineHeight: '2.5rem' }], // 36px
        xl: ['3rem', { lineHeight: '1' }], // 48px
        '2xl': ['3.75rem', { lineHeight: '1' }], // 60px
      },
    },
  },
  plugins: [],
}

