<script>
	import { upload } from '$lib/stores/upload';
	let { src, code, path, local } = $props();
	let is_dragging = $state(false);
	let is_downloading = $derived($upload.code == code);

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
