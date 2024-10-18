const baseURL = import.meta.env.MODE === 'development' ? 'http://127.0.0.1:3001' : ''

export default () => ({
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
    await fetch(`${baseURL}/clients/${uuid}/update/hostname/${new_name}`);
    const dialog = document.getElementById('dialog-rename');
    dialog.hide();
  },
  deleteDialog(uuid, hostname) {
    const dialog = document.getElementById('dialog-delete');
    const nameSpan = dialog.querySelector('span');
    nameSpan.innerText = hostname;
    const deleteButton = dialog.querySelector('sl-button[slot="footer"]');
    deleteButton.addEventListener('click', () => {
      this.delete(uuid);
    });
    dialog.show();
  },
  async delete(uuid) {
    await fetch(`${baseURL}/clients/${uuid}/delete`);
    const dialog = document.getElementById('dialog-delete');
    dialog.hide();
  },
  truncate(hostname, l) {
    return hostname.length > l ? hostname.substring(0, l) + "..." : hostname
  }
})
