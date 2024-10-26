<script>
	import { purity, categories, query } from '$lib/stores/searchparams';
	import ToggleButton from './ToggleButton.svelte';
	let { settings, sorting } = $props();
	let authed = $derived(settings.wallhaven_apikey != '');
	let search_term = $state('');

	function doSearch() {
		if (search_term !== '') {
			$query = search_term;
		}
	}

	function handleKeys(event) {
		if (event.key === 'Enter') {
			doSearch();
		}
	}
</script>

<div class="search-params-bar">
	<div class="button-group purity">
		<ToggleButton label="sfw" bind:checked={$purity.sfw} --color="#4b934e" />
		<ToggleButton label="sketchy" bind:checked={$purity.sketchy} --color="#F0E68C" />
		{#if authed}
			<ToggleButton label="nsfw" bind:checked={$purity.nsfw} --color="red" />
		{/if}
	</div>

	<div class="button-group categories">
		<ToggleButton label="general" bind:checked={$categories.general} --color="blue" />
		<ToggleButton label="anime" bind:checked={$categories.anime} --color="blue" />
		<ToggleButton label="people" bind:checked={$categories.people} --color="blue" />
	</div>

	{#if sorting == 'search'}
		<div class="searchbar">
			<input type="text" bind:value={search_term} onkeypress={handleKeys} />
			<button onclick={doSearch}>search</button>
		</div>
	{/if}
</div>

<style>
	.search-params-bar {
		display: flex;
		margin-bottom: 24px;
	}

	.button-group {
		display: flex;
		margin-right: 20px;
	}
</style>
