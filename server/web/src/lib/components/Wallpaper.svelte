<script>
	let { src, id, local } = $props();
</script>

<div class="wallpaper">
	{#if !local}
		<img {src} alt={id} />
	{/if}
</div>

<style>
	.wallpaper {
		width: 270px;
	}

	.wallpaper img {
		width: 100%;
		aspect-ratio: 16/10;
		object-fit: cover;
	}
</style>
