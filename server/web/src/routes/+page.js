import { base } from '$app/paths';

export async function load({ fetch }) {
  let app_base = base;
  if (!import.meta.env.PROD) {
    app_base = "http://127.0.0.1:3001"
  }

  let response = await fetch(`${app_base}/clients`)
  let data = await response.json()
  return { clients: data.clients }
}
