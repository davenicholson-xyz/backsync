<script>
	let { src, close } = $props();
</script>

<div class="wallpaper-preview" onclick={close}>
	<img {src} alt={src} />
</div>

<style>
	.wallpaper-preview {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		z-index: 1000;
		display: flex;
		align-items: center;
		justify-content: center;
		background: black;
	}

	.wallpaper-preview img {
		border: 6px solid rgba(255, 255, 255, 0.4);
		max-width: 90%;
		max-height: 90%;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
	}
</style>
