<script>
	import Icon from '@iconify/svelte';
import { settings } from "$lib/stores/settings"

let { wallhaven_username, wallhaven_apikey} = $settings;
let saving = $state(false)

async function saveChanges() {
  saving = true
		try {
			let response = await fetch(`${$settings.baseURL}/settings`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ wallhaven_username, wallhaven_apikey })
			});

			if (!response.ok) {
				const errData = await response.json();
				console.error(errData);
				throw new Error(`Error: ${response.status} - ${errData.message || 'Unknow error'}`);
			}

			let data = await response.json();
			return data;
		} catch (e) {
			console.error('Failed to update settings:', e.message);
			return { success: false, message: error.message };
		} finally {
        saving = false
      }
  }

</script>
<div>
	<h1>Settings</h1>
  <div>
  <label for="username">Wallhaven username: </label>
  <input type="text" bind:value={wallhaven_username} />
  </div>

  <div>
  <label for="apikey">Wallhaven API key: </label>
  <input type="text" bind:value={wallhaven_apikey} />
  </div>

  <div>
  <button onclick={saveChanges} band:disabled={saving}>
  <Icon icon="entypo:save" />
  Save
  </button>
  </div>

</div>

