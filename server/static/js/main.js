const clients = () => {
  return {
    data: [],
    fetch_clients() {
      fetch("/streams").then(response => response.json()).then(data => this.data = data.streams)
    }
  }
}
