<script>
	import { upload } from '$lib/stores/upload';
	let { src, code, path, link = '', local } = $props();
	let is_dragging = $state(false);
	let is_downloading = $derived($upload.code == code);

	function gotoLink() {}

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
	<img {src} alt={code} />

	<div class="actions">
		{#if link}
			<a href={link} target="_blank" aria-label="wallpaper link">
				<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
					><g fill="none"
						><path
							d="m12.593 23.258l-.011.002l-.071.035l-.02.004l-.014-.004l-.071-.035q-.016-.005-.024.005l-.004.01l-.017.428l.005.02l.01.013l.104.074l.015.004l.012-.004l.104-.074l.012-.016l.004-.017l-.017-.427q-.004-.016-.017-.018m.265-.113l-.013.002l-.185.093l-.01.01l-.003.011l.018.43l.005.012l.008.007l.201.093q.019.005.029-.008l.004-.014l-.034-.614q-.005-.018-.02-.022m-.715.002a.02.02 0 0 0-.027.006l-.006.014l-.034.614q.001.018.017.024l.015-.002l.201-.093l.01-.008l.004-.011l.017-.43l-.003-.012l-.01-.01z"
						/><path
							fill="currentColor"
							d="m17.303 9.524l3.182 3.182a5.5 5.5 0 1 1-7.778 7.778l-1.06-1.06a1.5 1.5 0 1 1 2.12-2.122l1.062 1.061a2.5 2.5 0 0 0 3.535-3.536l-3.182-3.182a2.5 2.5 0 0 0-2.681-.56q-.242.096-.454.196l-.464.217c-.62.28-1.097.4-1.704-.206c-.872-.872-.646-1.677.417-2.41a5.5 5.5 0 0 1 7.007.642m-6.01-6.01l1.06 1.06a1.5 1.5 0 0 1-2.12 2.122l-1.061-1.06A2.5 2.5 0 1 0 5.636 9.17l3.182 3.182a2.5 2.5 0 0 0 2.681.56q.242-.096.454-.196l.464-.217c.62-.28 1.098-.4 1.704.206c.872.872.646 1.677-.417 2.41a5.5 5.5 0 0 1-7.007-.642l-3.182-3.182a5.5 5.5 0 1 1 7.778-7.778Z"
						/></g
					></svg
				>
			</a>
		{/if}
	</div>

	{#if is_downloading}
		<div class="downloading">
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
				><g
					fill="none"
					stroke="currentColor"
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					><path
						stroke-dasharray="2 4"
						stroke-dashoffset="6"
						d="M12 3c4.97 0 9 4.03 9 9c0 4.97 -4.03 9 -9 9"
						><animate
							attributeName="stroke-dashoffset"
							dur="0.6s"
							repeatCount="indefinite"
							values="6;0"
						/></path
					><path
						stroke-dasharray="32"
						stroke-dashoffset="32"
						d="M12 21c-4.97 0 -9 -4.03 -9 -9c0 -4.97 4.03 -9 9 -9"
						><animate
							fill="freeze"
							attributeName="stroke-dashoffset"
							begin="0.1s"
							dur="0.4s"
							values="32;0"
						/></path
					><path stroke-dasharray="10" stroke-dashoffset="10" d="M12 8v7.5"
						><animate
							fill="freeze"
							attributeName="stroke-dashoffset"
							begin="0.5s"
							dur="0.2s"
							values="10;0"
						/></path
					><path stroke-dasharray="6" stroke-dashoffset="6" d="M12 15.5l3.5 -3.5M12 15.5l-3.5 -3.5"
						><animate
							fill="freeze"
							attributeName="stroke-dashoffset"
							begin="0.7s"
							dur="0.2s"
							values="6;0"
						/></path
					></g
				></svg
			>
		</div>
	{/if}
</div>

<style>
	.wallpaper {
		position: relative;
		width: 250px;
	}

	.actions {
		padding-top: 10px;
		position: absolute;
		top: 0;
		height: 100%;
		background: rgba(0, 0, 0, 0.5);
		display: grid;
		align-content: start;
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

	.actions a svg {
		width: 18px;
	}

	.actions button:hover {
		color: green;
		cursor: pointer;
	}

	.wallpaper img {
		width: 100%;
		aspect-ratio: 16/10;
		object-fit: cover;
		border-radius: 10px;
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
	}

	.wallpaper .downloading svg {
		width: 100px;
		opacity: 0.6;
	}
</style>
