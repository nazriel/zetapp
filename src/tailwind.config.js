/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./**/*.{js,ts,jsx,tsx}"],
  darkMode: "media",
  theme: {
    extend: {
      screens: {
        'portrait': {
          'raw': '(orientation: portrait)'
        },
        'landscape': {
          'raw': '(orientation: landscape)'
        },
      }
    },
  },
  daisyui: {
    themes: [
      "light",
      "dark",
      "cupcake",
    ],
    darkTheme: "dark",
    base: true,
    styled: true,
    utils: true,
    prefix: "",
    logs: true,
    themeRoot: ":root",
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
}
