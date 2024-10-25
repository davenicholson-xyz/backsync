<script>
	import WallhavenGallery from '$lib/components/WallhavenGallery.svelte';

	let { data } = $props();
	let { settings } = data;

	let url = $state(
		`https://wallhaven.cc/api/v1/search?ratios=landscape&sorting=toplist&purity=111&apikey=${settings.wallhaven_apikey}`
	);
</script>

<WallhavenGallery {url} {settings} />
