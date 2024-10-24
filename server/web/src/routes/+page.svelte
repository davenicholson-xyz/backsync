<script>
	import Gallery from '$lib/components/Gallery.svelte';
	import { makeSeed } from '$lib/utils';

	let { data } = $props();
	let { settings } = data;

	let seed = makeSeed(6);

	let url = $state(
		`https://wallhaven.cc/api/v1/search?q=boobs&ratios=landscape&seed=${seed}&sorting=random&purity=111&apikey=${settings.api_key}`
	);
</script>

<Gallery {url} {settings} />
