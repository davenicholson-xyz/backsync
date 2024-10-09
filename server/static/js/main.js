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
}

const wallpapers = () => {
  return {
    data: [],
    fetch_wallpapers() {
      fetch("/wallpapers").then(response => response.json()).then(data => this.data = data.wallpapers)
    }
  }
}

