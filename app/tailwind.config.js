/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["src/**/*.rs", "index.html"],
  theme: {
    extend: {
      colors: {
        navy: "#13161f",
        surf: "#1a1e2a",
        surf2: "#1f2433",
        surf3: "#252a3a",
        ink: "#e8e6e0",
        body: "#a8a49e",
        mute: "#8a8780",
        gold: "#f2992e",
        hl: "rgba(255,255,255,0.06)",
        hl2: "rgba(255,255,255,0.1)",
      },
    },
  },
};
