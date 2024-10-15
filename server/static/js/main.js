function router() {
  return {
    currentPage: 'home',
    pageContent: '',

    navigate(page) {
      this.currentPage = page;
      this.loadPageContent(page);
    },

    async loadPageContent(page) {
      console.log('navgating');
      const response = await fetch(`/static/pages/${page}.html`);
      this.pageContent = await response.text();
    },

    init() {
      this.loadPageContent(this.currentPage);
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
    console.log("WebSocket connection");
  };

  socket.onerror = function(error) {
    console.log('WebSocket Error: ', error);
  };

  socket.onclose = function() {
    console.log('WebSocket connection closed');
  };

});

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

