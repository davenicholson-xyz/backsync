<script>
	import Icon from '@iconify/svelte';

	import { clients, updating } from '$lib/stores/clients';
	import { settings } from '$lib/stores/settings';
	import { upload } from '$lib/stores/upload';

	let { client } = $props();
	let { hostname, wallpaper_code } = client;

	let dragover = $state(false);
	let is_setting = $state(false);

	let online = $derived(client.connected_at != '');
	let will_sync = $derived(client.syncwall != null);

	updating.subscribe((updates) => {
		if (updates.some((u) => u.uuid == client.uuid)) {
			is_setting = true;
		}
	});

	clients.subscribe((_) => {
		let update = $updating.find((u) => u.uuid == client.uuid);
		if (update) {
			if (update.code !== client.wallpaper_code) {
				updating.remove(client.uuid);
				is_setting = false;
			}
		}
	});

	function dragEnter(_) {
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
		updating.add(client.uuid, wallpaper.code);

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
			src={client.wallpaper_code
				? `${$settings.baseURL}/wallpapers/thumbnail/${client.wallpaper_code}`
				: '/missing.jpeg'}
			alt={`thumb-${wallpaper_code}`}
			class:dragover
			class:online
		/>

		{#if is_setting}
			<div class="client-setting">
				<Icon icon="svg-spinners:blocks-scale" />
			</div>
		{/if}
	</div>
	<div class="client-info">
		<div class="icons">
			{#if will_sync}
				<Icon icon="fluent:arrow-sync-12-filled" width="18px" />
			{/if}

			{#if client.locked}
				<Icon icon="fontisto:locked" width="12px" />
			{/if}
		</div>

		<a href={`/clients/${client.uuid}`}>
			{hostname}
		</a>

		{#if online}
			<div class="status-online">
				<Icon icon="heroicons-outline:status-online" />
			</div>
		{/if}
	</div>
</div>

<style>
	.client {
		position: relative;
	}

	.client-image {
		position: relative;
	}

	.client-image img {
		width: 220px;
		aspect-ratio: 16/10;
		object-fit: cover;
		border-radius: 10px;
		outline: 4px solid black;
		border: 4px solid rgba(255, 255, 255, 0.3);
	}

	.client-image img.dragover {
		outline: 4px dashed rgba(255, 255, 255, 0.3);
	}

	.client-image img.dragover.online {
		outline: 4px dashed #7cb9e8;
	}

	.client-info {
		display: flex;
		justify-content: start;
		align-items: center;
		gap: 2px;
		min-height: 30px;
	}

	.client-info .icons {
		color: #777;
		display: flex;
		align-items: center;
		margin-right: 8px;
		gap: 3px;
	}

	.status-online {
		margin-left: auto;
		color: lightgreen;
		pointer-events: none;
		font-size: 20px;
		margin-top: 4px;
	}

	.client-setting {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: rgba(0, 0, 0, 0.5);
		display: grid;
		place-items: center;
		color: rgba(255, 255, 255, 0.5);
		font-size: 80px;
		z-index: 1000;
	}

	.client-info a {
		color: white;
		text-decoration: none;
	}

	.client-info a:hover {
		color: #f39c12;
	}
</style>
