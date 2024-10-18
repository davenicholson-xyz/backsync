import './style.scss'
import 'iconify-icon'
import '@shoelace-style/shoelace/dist/themes/dark.css'
import '@shoelace-style/shoelace/dist/shoelace.js'
import router from "./js/router"
import clients from "./js/clients"
import search from "./js/search"
import { setBasePath } from '@shoelace-style/shoelace/dist/utilities/base-path'
setBasePath('/node_modules/@shoelace-style/shoelace/dist/')

const baseURL = import.meta.env.MODE === 'development' ? 'http://127.0.0.1:3001' : ''

import Alpine from 'alpinejs'
window.Alpine = Alpine

document.addEventListener('alpine:init', () => {
  Alpine.store('clients', {
    data: []
  })

  Alpine.store('settings', {
  data: [],
    async init() {
      let response = await fetch(`${baseURL}/settings`)
      let data = await response.json()
      this.data = data
    }
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
  };

  socket.onerror = function(error) {
    console.log('WebSocket Error: ', error);
  };

  socket.onclose = function() {
    console.log('WebSocket connection closed');
  };

});

  Alpine.data('router', router);
  Alpine.data('clients', clients);
  Alpine.data('search', search);

Alpine.start()

