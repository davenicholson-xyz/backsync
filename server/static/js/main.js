const clients = () => {
  return {
    data: [],
    async fetch_clients() {
      await fetch("/clients").then(response => response.json()).then(data => {
        console.log(data);
        this.data = data.streams;
      })
    }
  }
};

const wallpapers = () => {
  return {
    wallpapers: [],
    selectedFile: null,
    handleFileUpload(e) {
      this.selectedFile = e.target.files[0];
    },
    async setWallpaper(id) {
      // console.log('setting image');
      await fetch(`/wallpapers/set/${id}`);
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

