const baseURL = import.meta.env.MODE === 'development' ? 'http://127.0.0.1:3001' : ''

export default () => ({

  hoveredUUID: null,
  uploadingUUID: null,
  uploadingUUIDs: [],

  truncate(hostname, l) {
    return hostname.length > l ? hostname.substring(0, l) + "..." : hostname
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
    await fetch(`${baseURL}/clients/${uuid}/update/hostname/${new_name}`)
    const dialog = document.getElementById('dialog-rename');
    dialog.hide();
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
    await fetch(`${baseURL}/clients/${uuid}/delete`)
    const dialog = document.getElementById('dialog-delete')
    dialog.hide()
  },
  async lock(uuid) {
    await fetch(`${baseURL}/clients/${uuid}/update/locked/1`)
  },
  async unlock(uuid) {
    await fetch(`${baseURL}/clients/${uuid}/update/locked/0`);
  },

  dragEnter(uuid) {
    this.hoveredUUID = uuid;
  },
  dragLeave() {
    this.hoveredUUID = null
  },
  async dragDrop(event, uuid) {
    this.uploadingUUID = uuid;
    let url = event.dataTransfer.getData('text/plain')
    let response = await fetch(`${baseURL}/wallhaven/upload`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url })
    })
    let data = await response.json()
    await fetch(`${baseURL}/clients/${this.hoveredUUID}/set/${data.code}`)
    this.hoveredUUID = null;
    this.uploadingUUID = null;
  }

})
