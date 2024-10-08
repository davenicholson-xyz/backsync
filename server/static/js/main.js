function fetchData() {
  return {
    data: null,
    fetchDataFromApi() {
      fetch("/images/clone")
        .then(response => response.json()).then(json => {
          this.data = json.message;
        }).catch(e => console.error("error fetching", e))
    }
  }
}
