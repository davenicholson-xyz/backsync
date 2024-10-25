<script>
	import Gallery from '$lib/components/Gallery.svelte';

	let { data } = $props();
	let { settings } = data;

	let url = $state(
		`https://wallhaven.cc/api/v1/search?ratios=landscape&sorting=toplist&purity=110&apikey=${settings.wallhaven_apikey}`
	);
</script>

<Gallery {url} {settings} />
