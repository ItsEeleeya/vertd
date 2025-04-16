import { writable } from "svelte/store";

export const showGradient = writable(true);
export const gradientColor = writable("");
export const goingLeft = writable(false);
export const dropping = writable(false);

export const effects = writable(true);
export const appearance = writable<"light" | "dark">("light");
