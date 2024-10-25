<script>
	import { makeSeed } from '$lib/utils';
	import WallhavenGallery from '$lib/components/WallhavenGallery.svelte';

	let { data } = $props();
	let { settings } = data;

	let query = $state('');
	let url = $state('');

	function search() {
		let q_encoded = encodeURI(query);
		let seed = makeSeed(6);
		url = `https://wallhaven.cc/api/v1/search?q=${q_encoded}&ratios=landscape&sorting=random&seed=${seed}&purity=111&apikey=${settings.wallhaven_apikey}`;
	}

	function handleKeys(event) {
		if (event.key === 'Enter') {
			search();
		}
	}
</script>

<div class="search-bar">
	<input type="text" bind:value={query} onkeypress={handleKeys} />
	<button onclick={search}>search</button>
</div>

{#if url !== ''}
	{#key url}
		<WallhavenGallery {url} {settings} />
	{/key}
{/if}
