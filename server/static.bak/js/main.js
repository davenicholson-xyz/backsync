function router() {
  return {
    currentPage: '',
    pageContent: '',

    navigate(page, url) {
      this.currentPage = page;
      this.loadPageContent(page);
      history.pushState({ page: page }, '', url);
    },

    async loadPageContent(page) {
      const response = await fetch(`/static/pages/${page}.html`);
      this.pageContent = await response.text();
    },

    handlePopState(event) {
      const page = event.state ? event.state.page : 'home';
      this.currentPage = page;
      this.loadPageContent(page);
    },

    init() {
      const path = window.location.pathname;

      if (path == '/' || path == '') {
        this.navigate('home', '/');
      } else {
        const page = path.replace('/', '');
        this.navigate(page, path);
      }

      window.addEventListener('popstate', (e) => this.handlePopState(e));
    }
  }
}

document.addEventListener('alpine:init', () => {
  Alpine.store('clients', {
    data: []
  })

  const socket = new WebSocket('ws://127.0.0.1:3002');

  socket.onmessage = function(event) {
    let data = JSON.parse(event.data);
    switch (data.subject) {
      case "clients_update":
        Alpine.store('clients').data = [...data.clients];
        break;
      default:
        break;
    }
  };

  socket.onopen = function() {
  };

  socket.onerror = function(error) {
    console.log('WebSocket Error: ', error);
  };

  socket.onclose = function() {
    console.log('WebSocket connection closed');
  };

});

const clients = () => {
  return {
    get data() {
      return this.$store.clients.data;
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
      await fetch(`/clients/${uuid}/update/hostname/${new_name}`);
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
      await fetch(`/clients/${uuid}/delete`);
      const dialog = document.getElementById('dialog-delete');
      dialog.hide();
    }
  }
}

const wallpapers = () => {
  return {
    wallpapers: [],
    selectedFile: null,
    handleFileUpload(e) {
      this.selectedFile = e.target.files[0];
    },
    async setWallpaper(id) {
      await fetch(`/wallpapers/set/${id}`);
      await new Promise(r => setTimeout(r, 2000));
    },
    async uploadImage() {

      if (!this.selectedFile) {
        alert("please sleect a file")
        return;
      }

      let formData = new FormData();
      formData.append('image', this.selectedFile);
      this.selectedFile = null;

      try {
        let response = await fetch("/wallpapers/upload", {
          method: "POST",
          body: formData
        });

        if (response.ok) {
          let new_image = await response.json();
          this.wallpapers.push(new_image);
        } else {
          alert('image upload failed');
        }

      } catch (e) {
        console.error('error uploading', e);
      }

    },
    async deleteImage(code, ext) {
      try {
        let response = await fetch(`/wallpapers/delete/${code}.${ext}`, { method: "DELETE" });
        if (response.ok) {
          this.wallpapers = this.wallpapers.filter(w => w.code !== code);
        } else {
          alert("failed to delete image");
        }
      } catch (e) {
        console.error("error deleting image", e);
      }
    },
    fetchWallpapers() {
      fetch("/wallpapers").then(response => response.json()).then(data => this.wallpapers = data.wallpapers)
    }
  }
}

function dragAndDrop() {
  return {
    draggedWallpaper: null,
    hoveredClientId: null,

    dragStart(event) {
      this.draggedWallpaper = event.target.closest('[data-wallpaper-code]').getAttribute('data-wallpaper-code');
      const dragThumbnailImage = document.getElementById('drag-thumbnail-image');
      dragThumbnailImage.src = event.target.src;
      // dragThumbnailImage.src = event.target.querySelector('img').src;
      dragThumbnailImage.style.display = 'block';
      event.dataTransfer.setDragImage(dragThumbnailImage, 60, 25);
    },

    dragEnd() {
      this.draggedWallpaper = null;
      document.getElementById('drag-thumbnail-image').style.display = 'none';
      this.hoveredClientId = null;
    },

    dragEnter(clientId) {
      this.hoveredClientId = clientId;
    },

    dragLeave() {
      this.hoveredClientId = null;
    },

    async dropWallpaper(event) {
      const clientId = event.target.closest('[data-client-id]').getAttribute('data-client-id');
      if (clientId && this.draggedWallpaper) {
        await fetch(`/clients/${clientId}/set/${this.draggedWallpaper}`);
        await new Promise(r => setTimeout(r, 2000));
      }
    }
  }
}
