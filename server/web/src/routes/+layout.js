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
      clients.update(() => data.clients)
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


export async function load({ fetch }) {
  let baseURL = import.meta.env.PROD ? base : "http://127.0.0.1:3001"

  let default_settings = {
    wallhaven_apikey: null,
    wallhaven_username: null
  }

  try {
    let response = await fetch(`${baseURL}/settings`)
    let data = await response.json()
    default_settings = { ...default_settings, ...data }
  } catch (e) {
    console.error("could not get settings")
  }

  settings.set({ baseURL, ...default_settings, ...get(settings) })
  const current_settings = get(settings)
  return { settings: current_settings }
}

