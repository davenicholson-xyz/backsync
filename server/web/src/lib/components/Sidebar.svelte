<script>
	import Icon from '@iconify/svelte';
	import { page } from '$app/stores';

	let pathname = $derived($page.url.pathname);
	let sorting = $derived($page.url.search);
</script>

<div class="sidebar">
	<a href="/" class:active={pathname == '/'}>
		<Icon icon="fluent:home-12-filled" />
		<span>home</span>
	</a>
	<a href="/peers" class:active={pathname.startsWith('/peers')}>
		<Icon icon="basil:desktop-solid" />
		<span>peers</span>
	</a>
	<a href="/archive" class:active={pathname == '/archive'}>
		<Icon icon="zondicons:save-disk" />
		<span>archive</span>
	</a>

	<div class="spacer"></div>

	<h2>Wallhaven</h2>

	<a href="/wallhaven?sorting=search" class:active={sorting == '?sorting=search'}>
		<Icon icon="ic:baseline-image-search" />
		<span>search</span>
	</a>

	<a href="/wallhaven?sorting=hot" class:active={sorting == '?sorting=hot'}>
		<Icon icon="mdi:flame" />
		<span>hot</span>
	</a>

	<a href="/wallhaven?sorting=toplist" class:active={sorting == '?sorting=toplist'}>
		<Icon icon="flowbite:thumbs-up-solid" />
		<span>toplist</span>
	</a>

	<a href="/collections" class:active={pathname.startsWith('/collections')}>
		<Icon icon="icon-park-solid:collection-files" />
		<span>collections</span>
	</a>

	<div class="spacer"></div>

	<a href="/settings" class:active={pathname == '/settings'}>
		<Icon icon="fluent:settings-24-filled" />
		<span>settings</span>
	</a>
</div>

<style>
	.sidebar {
		padding: 20px;
		background-color: transparent;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.sidebar h2 {
		font-size: 18px;
		text-align: center;
		margin-top: 0px;
		margin-bottom: 4px;
		padding: 0;
		font-weight: normal;
	}

	.sidebar a {
		display: flex;
		align-items: center;
		gap: 8px;
		text-decoration: none;
		color: #ecf0f1;
		padding: 3px 15px;
		border-radius: 5px;
		font-size: 20px;
		transition:
			background-color 0.3s ease,
			color 0.3s ease;
	}
	.sidebar a span {
		font-size: 18px;
	}

	.sidebar a:hover {
		background-color: #34495e;
		color: #f39c12;
	}

	.sidebar a.active {
		color: #f39c12;
	}

	.sidebar .spacer {
		height: 50px;
	}
</style>
