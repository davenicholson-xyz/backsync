<script>
	import { purity, categories, query, toprange } from '$lib/stores/searchparams';
	import { page } from '$app/stores';
	import { makeSeed, convertToBinary } from '$lib/utils';

	import WallhavenGallery from '$lib/components/WallhavenGallery.svelte';
	import SearchParams from '$lib/components/SearchParams.svelte';

	let { data } = $props();
	let { settings } = data;

	let url = $state('');
	let apikey = settings.wallhaven_apikey != '' ? `&apikey=${settings.wallhaven_apikey}` : '';

	let sorting = $derived($page.url.searchParams.get('sorting'));

	let purity_s = $derived(convertToBinary($purity));
	let categories_s = $derived(convertToBinary($categories));

	let base_url = $derived(
		`https://wallhaven.cc/api/v1/search?ratios=landscape&purity=${purity_s}&categories=${categories_s}${apikey}`
	);

	$effect(() => {
		if ($query !== '') {
			let query_enc = encodeURI($query);
			let seed = makeSeed(6);
			url = `${base_url}&sorting=random&seed=${seed}&q=${query_enc}`;
		}
	});

	$effect(() => {
		switch (sorting) {
			case 'hot':
				url = `${base_url}&sorting=hot`;
				break;
			case 'toplist':
				url = `${base_url}&topRange=${$toprange}&sorting=toplist`;
				break;
			case 'search':
				url = '';
				break;
			default:
				url = ``;
				break;
		}
	});
</script>

<SearchParams {settings} {sorting} />

{#if url !== ''}
	{#key url}
		<WallhavenGallery {url} {settings} />
	{/key}
{/if}
