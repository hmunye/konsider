import { browser } from "$app/environment";
import { writable } from "svelte/store";

const defaultTheme = "dark";

const initialTheme = browser
  ? (window.localStorage.getItem("theme") ?? defaultTheme)
  : defaultTheme;

const theme = writable<string>(initialTheme);

theme.subscribe((value) => {
  if (browser) {
    window.localStorage.setItem("theme", value);
    document.documentElement.classList.toggle("dark", value === "dark");
    document.documentElement.classList.toggle("light", value === "light");
    document.documentElement.style.colorScheme = value;
  }
});

export { theme };
