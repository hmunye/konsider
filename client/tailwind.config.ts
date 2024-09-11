import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./src/**/*.{js,ts,jsx,tsx,mdx}"],
  theme: {
    extend: {
      fontFamily: {
        tanker: ["Tanker-Regular", "monospace"],
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
