<script>
	let { src, code, local } = $props();
	let is_dragging = $state(false);

	function dragStart(event) {
		is_dragging = true;
		let dragImg = document.getElementById('drag-thumbnail-image');
		dragImg.src = src;
		dragImg.style.display = 'block';
		event.dataTransfer.setDragImage(dragImg, 60, 25);

		event.dataTransfer.setData('application/json', JSON.stringify({ src, code }));
	}

	function dragEnd() {
		is_dragging = false;
		let dragImg = document.getElementById('drag-thumbnail-image');
		dragImg.style.display = 'none';
	}
</script>

<div class="wallpaper" draggable="true" ondragstart={dragStart} ondragend={dragEnd}>
	{#if !local}
		<img {src} alt={code} />
	{/if}
</div>

<style>
	.wallpaper {
		width: 290px;
	}

	.wallpaper img {
		width: 100%;
		aspect-ratio: 16/10;
		object-fit: cover;
		border-radius: 10px;
	}
</style>
