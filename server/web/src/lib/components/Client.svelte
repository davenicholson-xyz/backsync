<script>
	import { settings } from '$lib/stores/settings';
	import { upload } from '$lib/stores/upload';

	let { client } = $props();
	let { hostname, wallpaper_code } = client;

	let dragover = $state(false);
	let is_setting = $state(false);

	let online = $derived(client.connected_at != '');
	let will_sync = $derived(client.syncwall != null);

	function dragEnter(event) {
		dragover = true;
	}

	function dragLeave() {
		dragover = false;
	}

	async function dragDrop(event) {
		dragover = false;
		is_setting = true;

		let eData = event.dataTransfer.getData('application/json');
		let wallpaper = JSON.parse(eData);
		upload.set({ code: wallpaper.code });
		let wp = await uploadWallpaper(wallpaper.path);
		upload.set({ code: null });
		let set_wallpaper = await fetch(`${$settings.baseURL}/clients/${client.uuid}/set/${wp.code} `);
		is_setting = false;
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
		{#if is_setting}
			<div class="client-setting">
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
					><g fill="currentColor"
						><circle cx="12" cy="3.5" r="1.5"
							><animateTransform
								attributeName="transform"
								calcMode="discrete"
								dur="2.4s"
								repeatCount="indefinite"
								type="rotate"
								values="0 12 12;90 12 12;180 12 12;270 12 12"
							/><animate
								attributeName="opacity"
								dur="0.6s"
								repeatCount="indefinite"
								values="1;1;0"
							/></circle
						><circle cx="12" cy="3.5" r="1.5" transform="rotate(30 12 12)"
							><animateTransform
								attributeName="transform"
								begin="0.2s"
								calcMode="discrete"
								dur="2.4s"
								repeatCount="indefinite"
								type="rotate"
								values="30 12 12;120 12 12;210 12 12;300 12 12"
							/><animate
								attributeName="opacity"
								begin="0.2s"
								dur="0.6s"
								repeatCount="indefinite"
								values="1;1;0"
							/></circle
						><circle cx="12" cy="3.5" r="1.5" transform="rotate(60 12 12)"
							><animateTransform
								attributeName="transform"
								begin="0.4s"
								calcMode="discrete"
								dur="2.4s"
								repeatCount="indefinite"
								type="rotate"
								values="60 12 12;150 12 12;240 12 12;330 12 12"
							/><animate
								attributeName="opacity"
								begin="0.4s"
								dur="0.6s"
								repeatCount="indefinite"
								values="1;1;0"
							/></circle
						></g
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

		<span>{hostname}</span>
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
		border: 2px solid white;
	}

	.client-image img.dragover {
		border: 2px solid green;
	}

	.client-info {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.client-info .icons {
		color: #777;
		display: flex;
		align-items: center;
		justify-content: space-bwtween;
	}

	.client-info .status-dot {
		margin-left: -3px;
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
	}
</style>
