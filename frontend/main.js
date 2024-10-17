import './style.css'
import 'iconify-icon'
import '@shoelace-style/shoelace/dist/themes/dark.css'
import '@shoelace-style/shoelace/dist/shoelace.js'
import { setBasePath } from '@shoelace-style/shoelace/dist/utilities/base-path'
setBasePath('/node_modules/@shoelace-style/shoelace/dist/')

import Alpine from 'alpinejs'
window.Alpine = Alpine

const baseURL = import.meta.env.MODE === 'development' ? 'http://127.0.0.1:3001' : ''

Alpine.data('router', () => ({
  baseURL,
  currentPage: '',
  pageContent: '',

  navigate(page, url) {
    this.currentPage = page;
    this.loadPageContent(page);
    history.pushState({ page: page }, '', url);
  },

  async loadPageContent(page) {
    const response = await fetch(`/pages/${page}.html`);
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
}))

document.addEventListener('alpine:init', () => {
  Alpine.store('clients', {
    data: []
  })

  const socket = new WebSocket('ws://127.0.0.1:3002');

  socket.onmessage = function(event) {
    let data = JSON.parse(event.data);
    console.log(data);
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

Alpine.data('clients', () => ({
  get clients() {
    return this.$store.clients.data
  }
}))

Alpine.start()
