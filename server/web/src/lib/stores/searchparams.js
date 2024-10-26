import { writable } from "svelte/store";

let local_purity = localStorage.purity ? JSON.parse(localStorage.purity) : {
  sfw: true,
  sketchy: true,
  nsfw: false
}
export const purity = writable(local_purity)


let local_categories = localStorage.categories ? JSON.parse(localStorage.categories) : {
  general: true,
  anime: false,
  people: true
}
export const categories = writable(local_categories)

let local_toprange = localStorage.toprange ? localStorage.toprange : "1M"
export const toprange = writable(local_toprange)

purity.subscribe(v => localStorage.purity = JSON.stringify(v))
categories.subscribe(v => localStorage.categories = JSON.stringify(v))
toprange.subscribe(v => localStorage.toprange = v)

export const query = writable("")

