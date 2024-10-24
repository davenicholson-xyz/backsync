import { clients } from "../lib/stores/clients"
import { settings } from "../lib/stores/settings"
import { get } from "svelte/store";
import { base } from '$app/paths';

export const ssr = false

let socket = new WebSocket("ws://127.0.0.1:3002")

socket.onmessage = (event) => {
  let data = JSON.parse(event.data);
  switch (data.subject) {
    case "clients_update":
      clients.set(data.clients)
      break
    case "image_upload":
      break;
    default:
      break;
  }

  socket.onclose = () => {
    console.log("websocket closed")
  }

  socket.onerror = (e) => {
    console.error("socket error", e)
  }
}


export async function load() {
  let baseURL = import.meta.env.PROD ? base : "http://127.0.0.1:3001"
  let api_key = "XXMiYVopgEjJlslkFOWxkmbdM1k4nGEi"

  settings.set({ baseURL, api_key, ...get(settings) })
  const current_settings = get(settings)
  return { settings: current_settings }
}

