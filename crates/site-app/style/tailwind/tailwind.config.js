/** @type {import('tailwindcss').Config} */

const orbitComponentsPreset = require("@kiwicom/orbit-tailwind-preset");

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
  presets: [
    orbitComponentsPreset({
      disablePreflight: false, // default value
    }),
  ],
}
