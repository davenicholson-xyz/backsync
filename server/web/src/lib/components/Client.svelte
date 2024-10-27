<script>
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
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
					><rect width="10" height="10" x="1" y="1" fill="currentColor" rx="1"
						><animate
							id="svgSpinnersBlocksShuffle30"
							fill="freeze"
							attributeName="x"
							begin="0;svgSpinnersBlocksShuffle3b.end"
							dur="0.2s"
							values="1;13"
						/><animate
							id="svgSpinnersBlocksShuffle31"
							fill="freeze"
							attributeName="y"
							begin="svgSpinnersBlocksShuffle38.end"
							dur="0.2s"
							values="1;13"
						/><animate
							id="svgSpinnersBlocksShuffle32"
							fill="freeze"
							attributeName="x"
							begin="svgSpinnersBlocksShuffle39.end"
							dur="0.2s"
							values="13;1"
						/><animate
							id="svgSpinnersBlocksShuffle33"
							fill="freeze"
							attributeName="y"
							begin="svgSpinnersBlocksShuffle3a.end"
							dur="0.2s"
							values="13;1"
						/></rect
					><rect width="10" height="10" x="1" y="13" fill="currentColor" rx="1"
						><animate
							id="svgSpinnersBlocksShuffle34"
							fill="freeze"
							attributeName="y"
							begin="svgSpinnersBlocksShuffle30.end"
							dur="0.2s"
							values="13;1"
						/><animate
							id="svgSpinnersBlocksShuffle35"
							fill="freeze"
							attributeName="x"
							begin="svgSpinnersBlocksShuffle31.end"
							dur="0.2s"
							values="1;13"
						/><animate
							id="svgSpinnersBlocksShuffle36"
							fill="freeze"
							attributeName="y"
							begin="svgSpinnersBlocksShuffle32.end"
							dur="0.2s"
							values="1;13"
						/><animate
							id="svgSpinnersBlocksShuffle37"
							fill="freeze"
							attributeName="x"
							begin="svgSpinnersBlocksShuffle33.end"
							dur="0.2s"
							values="13;1"
						/></rect
					><rect width="10" height="10" x="13" y="13" fill="currentColor" rx="1"
						><animate
							id="svgSpinnersBlocksShuffle38"
							fill="freeze"
							attributeName="x"
							begin="svgSpinnersBlocksShuffle34.end"
							dur="0.2s"
							values="13;1"
						/><animate
							id="svgSpinnersBlocksShuffle39"
							fill="freeze"
							attributeName="y"
							begin="svgSpinnersBlocksShuffle35.end"
							dur="0.2s"
							values="13;1"
						/><animate
							id="svgSpinnersBlocksShuffle3a"
							fill="freeze"
							attributeName="x"
							begin="svgSpinnersBlocksShuffle36.end"
							dur="0.2s"
							values="1;13"
						/><animate
							id="svgSpinnersBlocksShuffle3b"
							fill="freeze"
							attributeName="y"
							begin="svgSpinnersBlocksShuffle37.end"
							dur="0.2s"
							values="1;13"
						/></rect
					></svg
				>
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
			<div class="status-dot" class:online>
				<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 48 48"
					><path
						fill="currentColor"
						stroke="currentColor"
						stroke-width="4"
						d="M24 33a9 9 0 1 0 0-18a9 9 0 0 0 0 18Z"
					/></svg
				>
			</div>
		</div>

		<a href={`/clients/${client.uuid}`}>
			{hostname}
		</a>
	</div>
</div>

<style>
	.client-image {
		position: relative;
	}

	.client-image img {
		width: 220px;
		aspect-ratio: 16/10;
		object-fit: cover;
		border-radius: 10px;
		outline: 4px solid black;
		border: 4px solid black;
	}

	.client-image img.dragover {
		outline: 4px dashed rgba(255, 255, 255, 0.3);
	}

	.client-image img.dragover.online {
		outline: 4px dashed #7cb9e8;
	}

	.client-info {
		display: flex;
		align-items: center;
		gap: 2px;
	}

	.client-info .icons {
		color: #777;
		display: flex;
		align-items: center;
		justify-content: space-bwtween;
	}

	.client-info .status-dot {
		margin-left: -3px;
		margin-right: -3px;
		padding-top: 4px;
	}

	.status-dot.online {
		color: #5ff83a;
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
	}

	.client-setting svg {
		width: 30%;
		color: rgba(255, 255, 255, 0.6);
	}

	.client-info a {
		color: white;
		text-decoration: none;
	}

	.client-info a:hover {
		color: #f39c12;
	}
</style>
