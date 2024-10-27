<script>
	import { purity, categories, query, toprange } from '$lib/stores/searchparams';
	import ToggleButton from './ToggleButton.svelte';
	import RadioButton from './RadioButton.svelte';

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
		<ToggleButton label="sfw" bind:checked={$purity.sfw} --color="#4b934e" klass="first" />
		<ToggleButton
			label="sketchy"
			bind:checked={$purity.sketchy}
			--color="#cec00f"
			klass={!authed ? 'last' : ''}
		/>
		{#if authed}
			<ToggleButton label="nsfw" bind:checked={$purity.nsfw} --color="#C70039" klass="last" />
		{/if}
	</div>

	<div class="button-group categories">
		<ToggleButton
			label="general"
			bind:checked={$categories.general}
			--color="#009ac7"
			klass="first"
		/>
		<ToggleButton label="anime" bind:checked={$categories.anime} --color="#009ac7" />
		<ToggleButton label="people" bind:checked={$categories.people} --color="#009ac7" klass="last" />
	</div>

	{#if sorting == 'toplist'}
		<div class="button-group toprange">
			<RadioButton
				label="1 day"
				value="1d"
				bind:group={$toprange}
				--color="#4b934e"
				klass="first"
			/>
			<RadioButton label="3 days" value="3d" bind:group={$toprange} --color="#4b934e" />
			<RadioButton label="1 week" value="1w" bind:group={$toprange} --color="#4b934e" />
			<RadioButton label="1 month" value="1M" bind:group={$toprange} --color="#4b934e" />
			<RadioButton label="3 months" value="3M" bind:group={$toprange} --color="#4b934e" />
			<RadioButton label="6 months" value="6M" bind:group={$toprange} --color="#4b934e" />
			<RadioButton
				label="1 year"
				value="1y"
				bind:group={$toprange}
				--color="#4b934e"
				klass="last"
			/>
		</div>
	{/if}

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
		margin-bottom: 32px;
	}

	.button-group {
		display: flex;
		margin-right: 20px;
	}
</style>
