const baseURL = import.meta.env.MODE === 'development' ? 'http://127.0.0.1:3001' : ''

export default () => ({
  baseURL,
  currentPage: '',
  pageContent: '',

  navigate(page, url) {
    this.currentPage = page;
    this.loadPageContent(page);
    history.pushState({ page: page }, '', url);
  },

  async loadPageContent(page) {
    try {
      const response = await fetch(`/pages/${page}.html`);
      console.log(response)
      if (response.status === 404) {
        this.pageContent = '404 not found'
        return
      }
      this.pageContent = await response.text();
    } catch (e) {
      console.error("error loading page content", e.message)
      this.pageContent = "error abound"
    }
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
})
