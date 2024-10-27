import { writable } from "svelte/store"
export const clients = writable([])

function createUpdating() {
  const { subscribe, set, update } = writable([])
  return {
    subscribe, set, update,
    add: (uuid, code) => update(u => [...u, { uuid, code }]),
    remove: (uuid) => update(u => u.filter(i => i.uuid != uuid))
  }
}

export const updating = createUpdating()

