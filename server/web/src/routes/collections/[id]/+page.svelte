<script>
	import Gallery from '$lib/components/Gallery.svelte';

	let { data } = $props();
	let { settings, id } = data;

	let url = $state(
		`https://wallhaven.cc/api/v1/collections/fatnic/${id}?apikey=${settings.api_key}`
	);
</script>

<Gallery {url} {settings} />
