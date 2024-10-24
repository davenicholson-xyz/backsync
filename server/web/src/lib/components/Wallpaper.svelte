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
		width: 290px;
	}

	.wallpaper img {
		width: 100%;
		aspect-ratio: 16/10;
		object-fit: cover;
		border-radius: 10px;
	}
</style>
