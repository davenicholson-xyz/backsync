// const button = document.querySelector("button");
// button.addEventListener('click', (e) => {
//   e.target.disabled = true;
// })

const clients = () => {
  return {
    data: [],
    fetch_clients() {
      fetch("/streams").then(response => response.json()).then(data => this.data = data.streams)
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
    async uploadImage() {

      if (!this.selectedFile) {
        alert("please sleect a file")
        return;
      }

      let formData = new FormData();
      formData.append('image', this.selectedFile);

      try {
        let response = await fetch("/wallpapers/upload", {
          method: "POST",
          body: formData
        });

        if (response.ok) {
          let new_image = await response.json();
          this.wallpapers.push(new_image);
          this.selectedFile = null;
        } else {
          alert('image upload failed');
        }

      } catch (e) {
        console.error('error uploading', e);
      }

    },
    async deleteImage(id, ext) {
      try {
        let response = await fetch(`/wallpapers/delete/${id}.${ext}`, { method: "DELETE" });
        if (response.ok) {
          this.wallpapers = this.wallpapers.filter(w => w.id !== id);
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

