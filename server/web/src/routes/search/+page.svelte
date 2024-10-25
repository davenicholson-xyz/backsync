<script>
	import { makeSeed } from '$lib/utils';
	import Gallery from '$lib/components/Gallery.svelte';

	let { data } = $props();
	let { settings } = data;

	let query = $state('');
	let url = $state('');

	function search() {
		let q_encoded = encodeURI(query);
		let seed = makeSeed(6);
		url = `https://wallhaven.cc/api/v1/search?q=${q_encoded}&ratios=landscape&sorting=random&seed=${seed}&purity=111&apikey=${settings.wallhaven_apikey}`;
		console.log(url);
	}
</script>

<div class="search-bar">
	<input type="text" bind:value={query} />
	<button onclick={search}>search</button>
</div>

{#if url !== ''}
	{#key url}
		<Gallery {url} {settings} />
	{/key}
{/if}
