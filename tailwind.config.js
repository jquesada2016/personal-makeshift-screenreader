/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "index.html",
    "wasm/src/**/*.rs",
    "wasm/wasmdaisyui_component_classes.txt",
  ],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
};
