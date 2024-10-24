<script>
	import Gallery from '$lib/components/Gallery.svelte';
	import { makeSeed } from '$lib/utils';

	let { data } = $props();
	let { settings } = data;

	let seed = makeSeed(6);

	let url = $state(
		`https://wallhaven.cc/api/v1/search?q=&ratios=landscape&seed=${seed}&sorting=random&purity=110&apikey=${settings.api_key}`
	);
</script>

<Gallery {url} {settings} />
