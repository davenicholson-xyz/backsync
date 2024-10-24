<script>
	import { settings } from '../stores/settings';
	let { client } = $props();
	let { hostname, wallpaper_code } = client;
</script>

<div class="client">
	<div class="client-image">
		<img
			src={`${$settings.baseURL}/wallpapers/thumbnail/${wallpaper_code}`}
			alt={`thumb-${wallpaper_code}`}
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
</style>
