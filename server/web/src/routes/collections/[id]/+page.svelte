<script>
	import WallhavenGallery from '$lib/components/WallhavenGallery.svelte';

	let { data } = $props();
	let { settings, id } = data;

	let url = $state(
		`https://wallhaven.cc/api/v1/collections/${settings.wallhaven_username}/${id}?apikey=${settings.wallhaven_apikey}`
	);
</script>

<h2>Collection Name</h2>

<WallhavenGallery {url} {settings} />
