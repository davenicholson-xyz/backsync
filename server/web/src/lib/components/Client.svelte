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
        <Icon icon="blocks-scale" />
			</div>
		{/if}
	</div>
	<div class="client-info">
		<div class="icons">
			{#if will_sync}
				<div class="will_sync">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
						><path
							fill="currentColor"
							d="M3 12q0-3.75 2.625-6.375T12 3V2q0-.3.275-.45t.525.05l3.125 2.35q.4.3.4.8t-.4.8L12.8 7.9q-.25.2-.525.05T12 7.5v-1q-2.275 0-3.888 1.613T6.5 12q0 .825.238 1.588T7.4 15q.275.4.225.863T7.2 16.6l-.85.625q-.45.35-1 .275t-.875-.55q-.725-1.075-1.1-2.325T3 12m9 9v1q0 .3-.275.45t-.525-.05l-3.125-2.35q-.4-.3-.4-.8t.4-.8L11.2 16.1q.25-.2.525-.05t.275.45v1q2.275 0 3.888-1.613T17.5 12q0-.825-.238-1.588T16.6 9q-.275-.4-.225-.862T16.8 7.4l.85-.625q.45-.35 1-.263t.875.538q.7 1.075 1.088 2.325T21 12q0 3.75-2.625 6.375T12 21"
						/></svg
					>
				</div>
			{/if}

			{#if client.locked}
				<div class="locked">
					<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 8 8"
						><path
							fill="currentColor"
							d="M4 0C2.9 0 2 .9 2 2v1H1v4h6V3H6V2c0-1.1-.9-2-2-2m0 1c.56 0 1 .44 1 1v1H3V2c0-.56.44-1 1-1"
						/></svg
					>
				</div>
			{/if}
		</div>

		<a href={`/clients/${client.uuid}`}>
			{hostname}
		</a>

		{#if online}
			<div class="status-online">
				<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 26 26"
					><path
						fill="currentColor"
						d="M20.281 4.063a1.5 1.5 0 0 0-.906 2.593A8.94 8.94 0 0 1 22 13a8.95 8.95 0 0 1-2.625 6.344a1.503 1.503 0 1 0 2.125 2.125a11.995 11.995 0 0 0 0-16.938a1.5 1.5 0 0 0-1.063-.469a2 2 0 0 0-.156 0zm-14.906.03a1.5 1.5 0 0 0-.875.438a11.995 11.995 0 0 0 0 16.938a1.503 1.503 0 1 0 2.125-2.125A8.94 8.94 0 0 1 4 13a8.95 8.95 0 0 1 2.625-6.344a1.5 1.5 0 0 0-1.25-2.562zm3.813 3.313a1.5 1.5 0 0 0-.876.407A7 7 0 0 0 6 13c0 2.048.87 3.91 2.281 5.188a1.504 1.504 0 1 0 2.031-2.22A3.98 3.98 0 0 1 9 13a3.98 3.98 0 0 1 1.313-2.969a1.5 1.5 0 0 0-1.126-2.625zm7.406 0a1.5 1.5 0 0 0-.907 2.625A3.98 3.98 0 0 1 17 13a3.98 3.98 0 0 1-1.313 2.969a1.5 1.5 0 1 0 2 2.219A7 7 0 0 0 20 13a6.98 6.98 0 0 0-2.281-5.188a1.5 1.5 0 0 0-1.125-.406M13 11.188A1.812 1.812 0 1 0 14.813 13A1.81 1.81 0 0 0 13 11.187z"
					/></svg
				>
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
		justify-content: center;
		align-items: center;
		gap: 2px;
	}

	.client-info .icons {
		color: #777;
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-right: 6px;
	}

	.status-online {
		margin-left: auto;
		color: lightgreen;
		pointer-events: none;
	}

	.status-online svg {
		padding-top: 3px;
		width: 18px;
	}

	.will_sync {
		font-size: 12px;
		margin-left: 0px;
	}

	.will_sync svg {
		padding-top: 3px;
		width: 16px;
	}

	.locked {
		font-size: 12px;
		margin-left: 0px;
	}

	.locked svg {
		padding-top: 3px;
		width: 16px;
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
    color: rgba(255,255,255,0.5);
    font-size: 40px;
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
