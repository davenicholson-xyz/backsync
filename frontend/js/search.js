const baseURL = import.meta.env.MODE === 'development' ? 'http://127.0.0.1:3001' : ''

export default () => ({
  results: [],
  query: '',
  async search_for() {
    if (this.query.trim() == '') { return }

    this.results = []

    let q = encodeURI(this.query)
    let settings = Alpine.store('settings').data
    let url = `https://wallhaven.cc/api/v1/search?q=${q}&purity=111&categories=101&sorting=random`
    if (settings.wallhaven_apikey != "") {
      url = url + `&apikey=${settings.wallhaven_apikey}`
    }
    let response = await fetch(`${baseURL}/wallhaven/search`, {
      method: 'POST',
      headers: { 'Content-Type' : 'application/json' },
      body: JSON.stringify({url})
    })
    let data = await response.json()
    this.results = data.data
    console.log(data)
  },
  init() {
    let query = document.querySelector("#query")
    query.addEventListener('sl-input',(e) => {
      this.query = e.target.value;
    })
  }
}) 
