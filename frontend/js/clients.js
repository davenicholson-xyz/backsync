import { Alpine } from "alpinejs"

const baseURL = import.meta.env.MODE === 'development' ? 'http://127.0.0.1:3001' : ''

export default () => ({

  hoveredUUID: null,

  truncate(hostname, l) {
    return hostname.length > l ? hostname.substring(0, l) + "..." : hostname
  },

  showSpinner(uuid, code) {
    Alpine.store('client_updates')
    let update = Alpine.store('client_updates').find(c => c.uuid === uuid)
    return update && update.code == code
  },

  menu(uuid) {
    let menu = document.getElementById(`menu-${uuid}`)
    menu.show()
  },

  renameDialog(uuid, hostname) {
    const dialog = document.getElementById('dialog-rename');
    const nameInput = dialog.querySelector('sl-input');
    nameInput.value = hostname;

    const renameButton = dialog.querySelector('sl-button[slot="footer"]');
    renameButton.addEventListener('click', () => {
      let value = nameInput.value;
      this.rename(uuid, value);
    });
    dialog.show();
  },

  async rename(uuid, new_name) {
    try {
      let rename = await fetch(`${baseURL}/clients/${uuid}/update/hostname/${new_name}`)
      if (!rename.ok) {
        throw new Error(`Request failed with status ${response.status}: ${response.statusText}`);
      }

      const dialog = document.getElementById('dialog-rename');
      dialog.hide();
    } catch (e) {
      console.error("failed to update client hostname:", e.message)
    }
  },

  deleteDialog(uuid, hostname) {
    const dialog = document.getElementById('dialog-delete')
    const nameSpan = dialog.querySelector('span')
    nameSpan.innerText = hostname
    const deleteButton = dialog.querySelector('sl-button[slot="footer"]')
    deleteButton.addEventListener('click', () => {
      this.delete(uuid)
    })
    dialog.show()
  },

  async delete(uuid) {
    try {
      let response = await fetch(`${baseURL}/clients/${uuid}/delete`)
      if (!response.ok) {
        throw new Error(`Request failed with status ${response.status}: ${response.statusText}`);
      }
      const dialog = document.getElementById('dialog-delete')
      dialog.hide()
    } catch (e) {
      console.error("failed to delete client:", e.message)
    }

  },

  async lock(uuid) {
    try {
      let response = await fetch(`${baseURL}/clients/${uuid}/update/locked/1`)
      if (!response.ok) {
        throw new Error(`Request failed with status ${response.status}: ${response.statusText}`);
      }
    } catch (e) {
      console.error("failed to lock client:", e.message)
    }
  },

  async unlock(uuid) {
    try {
      let response = await fetch(`${baseURL}/clients/${uuid}/update/locked/0`);
      if (!response.ok) {
        throw new Error(`Request failed with status ${response.status}: ${response.statusText}`);
      }
    } catch (e) {
      console.error("failed to unlock client:", e.message)
    }
  },

  dragEnter(uuid) {
    this.hoveredUUID = uuid;
  },

  dragLeave() {
    this.hoveredUUID = null
  },

  addToUpdating(uuid) {
    let client = Alpine.store('clients').find(c => c.uuid === uuid)
    let currentUpdates = Alpine.store('client_updates')
    Alpine.store('client_updates', [...currentUpdates, { 'uuid': uuid, code: client.wallpaper_code }])
  },


  async uploadWallpaper(url) {
    try {
      let response = await fetch(`${baseURL}/wallhaven/upload`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ url })
      })

      if (!response.ok) {
        const errData = await response.json()
        console.error(errData)
        throw new Error(`Error: ${response.status} - ${errData.message || "Unknow error"}`)
      }

      let data = await response.json()
      return data

    } catch (e) {
      console.error("Failed to upload wallpaper:", e.message)
      return { success: false, message: error.message }
    }
  },

  // async dropAll(event) {
  //   let url = event.dataTransfer.getData('text/plain')
  //   let clients = Alpine.store('clients').filter(c => c.locked == false)
  //   clients.forEach(client => {
  //     if (client.connected_at != "") {
  //       this.addToUpdating(client.uuid)
  //     }
  //   })
  //   let wp = await this.uploadWallpaper(url)
  //   clients.forEach(async client => {
  //     await fetch(`${baseURL}/clients/${client.uuid}/set/${wp.code} `)
  //   })
  // },
  //

  async dragDrop(event, uuid) {
    event.preventDefault()

    try {
      let eData = event.dataTransfer.getData('application/json')
      let wallpaper = JSON.parse(eData)
      this.hoveredUUID = null

      let client = Alpine.store('clients').find(c => c.uuid === uuid)
      let current_wp = null

      if (client.locked) {
        return;
      }

      if (client.wallpaper_code) {
        let response = await fetch(`${baseURL}/wallpapers/code/${client.wallpaper_code}`)
        if (!response.ok) {
          throw new Error(`Request failed with status ${response.status}: ${response.statusText}`);
        }
        let data = await response.json()
        current_wp = data.origin
      }

      if (wallpaper.path === current_wp) { return }

      if (client.connected_at != "") {
        this.addToUpdating(uuid)
      }

      Alpine.store('upload').id = wallpaper.id
      let wp = await this.uploadWallpaper(wallpaper.path)
      if (!wp || !wp.code) {
        throw new Error('Failed to upload wallpaper')
      }

      let set_wallpaper = await fetch(`${baseURL}/clients/${client.uuid}/set/${wp.code} `)
      if (!set_wallpaper.ok) {
        throw new Error('Failed to set client wallpaper:', set_wallpaper.statusText)
      }

    } catch (e) {
      console.error('Error during drag-drop:', e.message)
      this.hoveredUUID = null;
    }

  },

})
