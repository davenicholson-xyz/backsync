<script>
	import Icon from '@iconify/svelte';
	import { upload } from '$lib/stores/upload';
	import { settings } from '$lib/stores/settings';

	let { src, code, path, link = '', local, remove, preview } = $props();
	let is_dragging = $state(false);
	let is_downloading = $derived($upload.code == code);

	async function del() {
		try {
			let response = await fetch(`${$settings.baseURL}/wallpapers/delete/${code}`, {
				method: 'DELETE'
			});
			if (response.ok) {
				remove(code);
			}
		} catch (e) {
			console.error(e);
		}
	}

	function dragStart(event) {
		is_dragging = true;
		let dragImg = document.getElementById('drag-thumbnail-image');
		dragImg.src = src;
		dragImg.style.display = 'block';
		event.dataTransfer.setDragImage(dragImg, 60, 25);

		event.dataTransfer.setData('application/json', JSON.stringify({ path, code }));
	}

	function dragEnd() {
		is_dragging = false;
		let dragImg = document.getElementById('drag-thumbnail-image');
		dragImg.style.display = 'none';
	}
</script>

<div
	class="wallpaper"
	draggable="true"
	ondragstart={dragStart}
	ondragend={dragEnd}
	role="application"
>
	<img {src} alt={code} onclick={() => preview(path)} />

	<div class="actions">
		{#if link}
			<a href={link} target="_blank" aria-label="wallpaper link">
				<Icon icon="mingcute:link-fill" />
			</a>
		{/if}
		<button aria-label="set-all">
			<Icon icon="solar:gallery-download-bold" />
		</button>
		{#if local}
			<button aria-label="delete" class="delete" onclick={del}>
				<Icon icon="material-symbols:delete" />
			</button>
		{/if}
	</div>

	{#if is_downloading}
		<div class="downloading">
			<Icon icon="line-md:downloading-loop" />
		</div>
	{/if}
</div>

<style>
	.wallpaper {
		position: relative;
		width: 250px;
		overflow: hidden;
	}

	.actions {
		position: absolute;
		top: 0;
		height: 100%;
		background: rgba(0, 0, 0, 0.5);
		display: grid;
		gap: 6px;
		align-content: center;
		opacity: 0;
		transform: translateX(-100%);
		transition: all 0.5s;
	}

	.wallpaper:hover .actions {
		opacity: 1;
		transform: translateX(0);
	}

	.actions a {
		padding-inline: 8px;
		background: transparent;
		color: white;
	}

	.actions button {
		background: none;
		border: none;
		cursor: pointer;
		color: white;
		margin: 0;
		padding: 0 6px;
	}

	.actions button:hover {
		color: #f39c12;
	}

	.actions button.delete:hover {
		color: red;
	}

	.actions a:hover {
		color: #f39c12;
	}

	.actions {
		font-size: 18px;
	}

	.wallpaper img {
		width: 100%;
		aspect-ratio: 16/10;
		object-fit: cover;
		border-radius: 4px;
		cursor: pointer;
	}

	.wallpaper .downloading {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: grid;
		place-items: center;
		background: rgba(0, 0, 0, 0.5);
		font-size: 100px;
		color: rgba(255, 255, 255, 0.5);
	}
</style>
