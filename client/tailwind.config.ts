import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        nippo: ["var(--font-nippo)", "sans-serif"],
      },
      colors: {
        foreground: "rgb(var(--foreground))",
        background: "rgb(var(--background))",
        primary: "rgb(var(--primary))",
        secondary: "rgb(var(--secondary))",
        accent: "rgb(var(--accent))",
        destructive: "rgb(var(--destructive))",
        muted: "rgb(var(--muted))",
      },
    },
  },
  plugins: [],
};
export default config;
