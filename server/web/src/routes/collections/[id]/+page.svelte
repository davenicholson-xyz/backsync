<script>
	import Gallery from '$lib/components/Gallery.svelte';

	let { data } = $props();
	let { settings, id } = data;

	let url = $state(
		`https://wallhaven.cc/api/v1/collections/${settings.wallhaven_username}/${id}?apikey=${settings.wallhaven_apikey}`
	);
</script>

<Gallery {url} {settings} />
