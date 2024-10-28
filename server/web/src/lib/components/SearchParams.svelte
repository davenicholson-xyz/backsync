<script>
	import { purity, categories, query, toprange } from '$lib/stores/searchparams';
	import ToggleButton from './ToggleButton.svelte';
	import RadioButton from './RadioButton.svelte';
	import { onMount } from 'svelte';

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
			<input
				type="text"
				id="search-input"
				bind:value={search_term}
				onkeypress={handleKeys}
				autofocus
			/>
			<button onclick={doSearch}> search </button>
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

	.searchbar {
		display: flex;
	}
	.searchbar button {
		border-radius: 0 5px 5px 0px;
		border: none;
		background: #4b934e;
		color: white;
		text-shadow: 0px 1px 2px rgba(0, 0, 0, 0.5);
	}
	.searchbar button:hover {
		cursor: pointer;
	}

	input[type='text'] {
		background-color: #222;
		color: #fff;
		border: 1px solid #555;
		padding: 3px 15px;
		border-radius: 5px 0 0 5px;
		font-size: 18px;
		outline: none;
		width: 220px;
		transition:
			border-color 0.3s ease,
			box-shadow 0.3s ease;
	}

	input[type='text']:focus {
		border-color: #888;
		box-shadow: 0 0 5px rgba(255, 255, 255, 0.5);
	}
</style>
