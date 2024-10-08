const clients = () => {
  return {
    data: [
      { ip: "127.0.23.23", connected: false },
      { ip: "127.5.23.23", connected: true },
      { ip: "127.992.23.23", connected: true },
    ],
    fetch_clients() {
      console.log("fetch clients");
    }
  }
}
