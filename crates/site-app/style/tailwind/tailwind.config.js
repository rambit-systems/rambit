/** @type {import('tailwindcss').Config} */

const orbitComponentsPreset = require("@kiwicom/orbit-tailwind-preset");

module.exports = {
  content: [ "./crates/**/*.rs" ],
  theme: {
    extend: {
      fontFamily: {
        sans: ["sans-serif"],
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
