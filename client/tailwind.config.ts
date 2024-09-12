import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./src/**/*.{js,ts,jsx,tsx,mdx}"],
  theme: {
    extend: {
      fontFamily: {
        "nippo-extra-light": ["Nippo-ExtraLight", "monospace"],
        "nippo-light": ["Nippo-Light", "monospace"],
        "nippo-regular": ["Nippo-Regular", "monospace"],
        "nippo-medium": ["Nippo-Medium", "monospace"],
        "nippo-bold": ["Nippo-Bold", "monospace"],
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
