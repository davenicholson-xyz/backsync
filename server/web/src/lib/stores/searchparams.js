import { writable } from "svelte/store";

export const purity = writable({
  sfw: true,
  sketchy: true,
  nsfw: false
})

export const categories = writable({
  general: true,
  anime: false,
  people: true
})

export const query = writable("")

