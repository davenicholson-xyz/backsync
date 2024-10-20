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

  Alpine.store('clients', [])
  Alpine.store('client_updates', [])
  Alpine.store('upload', { progress: 0, id: null })
  Alpine.store('settings', {
    data: {},
    async init() {
      let response = await fetch(`${baseURL}/settings`)
      let data = await response.json()
      this.data = data
    }
  })

  Alpine.effect(() => {
    let clients = Alpine.store('clients');
    let clientUpdates = Alpine.store('client_updates');

    clients.forEach(client => {
      let cu = clientUpdates.find(u => u.uuid == client.uuid)
      if (cu) {
        if (cu.code != client.wallpaper_code) {
          let new_updates = clientUpdates.filter(u => u.uuid != cu.uuid);
          Alpine.store('client_updates', new_updates);
        }
      }
    })
  })

  Alpine.data('router', router);
  Alpine.data('clients', clients);
  Alpine.data('search', search);

  const socket = new WebSocket('ws://127.0.0.1:3002');

  socket.onmessage = function(event) {
    let data = JSON.parse(event.data);
    switch (data.subject) {
      case "clients_update":
        Alpine.store('clients', data.clients);
        break;
      case "image_upload":
        Alpine.store('upload').progress = data.progress;
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



Alpine.start()

