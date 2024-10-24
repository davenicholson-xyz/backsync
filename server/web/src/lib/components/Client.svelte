<script>
	import { settings } from '$lib/stores/settings';
	import { upload } from '$lib/stores/upload';
	import Wallpaper from './Wallpaper.svelte';

	let { client } = $props();
	let { hostname, wallpaper_code } = client;

	let dragover = $state(false);

	function dragEnter(event) {
		dragover = true;
	}

	function dragLeave() {
		dragover = false;
	}

	async function dragDrop(event) {
		dragover = false;
		let eData = event.dataTransfer.getData('application/json');
		let wallpaper = JSON.parse(eData);
		upload.set({ code: wallpaper.code });
		let wp = await uploadWallpaper(wallpaper.path);
		upload.set({ code: null });
		let set_wallpaper = await fetch(`${$settings.baseURL}/clients/${client.uuid}/set/${wp.code} `);
	}

	function dragOver(event) {
		event.preventDefault();
	}

	async function uploadWallpaper(url) {
		try {
			let response = await fetch(`${$settings.baseURL}/wallhaven/upload`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ url })
			});

			if (!response.ok) {
				const errData = await response.json();
				console.error(errData);
				throw new Error(`Error: ${response.status} - ${errData.message || 'Unknow error'}`);
			}

			let data = await response.json();
			return data;
		} catch (e) {
			console.error('Failed to upload wallpaper:', e.message);
			return { success: false, message: error.message };
		}
	}
</script>

<div class="client">
	<div class="client-image">
		<img
			ondragenter={dragEnter}
			ondragleave={dragLeave}
			ondrop={dragDrop}
			ondragover={dragOver}
			src={`${$settings.baseURL}/wallpapers/thumbnail/${client.wallpaper_code}`}
			alt={`thumb-${wallpaper_code}`}
			class:dragover
		/>
	</div>
	<div><span>{hostname}</span></div>
</div>

<style>
	.client-image img {
		width: 220px;
		aspect-ratio: 16/10;
		object-fit: cover;
		border-radius: 10px;
		border: 2px solid white;
	}

	.client-image img.dragover {
		border: 2px solid green;
	}
</style>
