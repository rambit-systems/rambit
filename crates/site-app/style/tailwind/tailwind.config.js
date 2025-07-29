/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [ "./crates/**/*.rs" ],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Funnel Sans", "sans-serif"],
        display: ["Funnel Display", "sans-serif"],
      },
      colors: {
        cloud: {
          light: "#F5F7F9",
          normal: "#E8EDF1",
          dark: "#BAC7D5",
        },
        ink: {
          light: "#657890",
          normal: "#4F5E71",
          dark: "#252A31",
        },
        product: "#F76B15",
      },
    },
  },
  plugins: [],
}
