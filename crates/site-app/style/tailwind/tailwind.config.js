/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [ "./crates/**/*.rs" ],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Funnel Sans", "sans-serif"],
        display: ["Funnel Display", "sans-serif"],
      },
    },
  },
  plugins: [],
}
