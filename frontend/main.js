import './style.css'
import '@shoelace-style/shoelace/dist/themes/dark.css'; // or dark.css for the dark theme
import '@shoelace-style/shoelace/dist/shoelace.js'; // Load all Shoelace components
import { setBasePath } from '@shoelace-style/shoelace/dist/utilities/base-path';
setBasePath('/node_modules/@shoelace-style/shoelace/dist/');

import Alpine from 'alpinejs'
window.Alpine = Alpine

const baseURL = import.meta.env.MODE === 'development' ? 'http://127.0.0.1:3001' : ''

Alpine.data('wallpapers', () => ({
  wallpapers: [],
  baseURL: baseURL,
  async fetchWallpapers() {
    let response = await fetch(`${baseURL}/wallpapers`)
    let data = await response.json()
    this.wallpapers = data.wallpapers
  },
  init() {
    this.fetchWallpapers()
    console.log(this.data);
  }
}));

Alpine.start()

