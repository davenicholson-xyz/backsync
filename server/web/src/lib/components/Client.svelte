<script>
	import { settings } from '../stores/settings';
	let { client } = $props();
	let { hostname, wallpaper_code } = client;

	let dragover = $state(false);

	function dragEnter(event) {
		dragover = true;
		console.log(event);
	}

	function dragLeave() {
		console.log('left');
		dragover = false;
	}

	function dragDrop(event) {
		let eData = event.dataTransfer.getData('application/json');
		let wallpaper = JSON.parse(eData);
		console.log(wallpaper);
		dragover = false;
	}

	function dragOver(event) {
		event.preventDefault();
	}
</script>

<div class="client">
	<div class="client-image">
		<img
			ondragenter={dragEnter}
			ondragleave={dragLeave}
			ondrop={dragDrop}
			ondragover={dragOver}
			src={`${$settings.baseURL}/wallpapers/thumbnail/${wallpaper_code}`}
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
