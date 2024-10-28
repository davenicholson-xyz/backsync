<script>
	import { page } from '$app/stores';
	import { get } from 'svelte/store';

	let pathname = $derived($page.url.pathname);
	let sorting = $derived($page.url.search);
</script>

<div class="sidebar">
	<a href="/" class:active={pathname == '/'}>
		<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
			><path
				fill="currentColor"
				stroke="currentColor"
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M20 19v-8.5a1 1 0 0 0-.4-.8l-7-5.25a1 1 0 0 0-1.2 0l-7 5.25a1 1 0 0 0-.4.8V19a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1v-3a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v3a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1"
			/></svg
		>
		<span>home</span>
	</a>
	<a href="/peers" class:active={pathname.startsWith('/peers')}>
		<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="-2 -2.5 24 24"
			><path
				fill="currentColor"
				d="M3 .565h14a3 3 0 0 1 3 3v9a3 3 0 0 1-3 3H3a3 3 0 0 1-3-3v-9a3 3 0 0 1 3-3m4 16h6a1 1 0 0 1 0 2H7a1 1 0 0 1 0-2"
			/></svg
		>
		<span>peers</span>
	</a>
	<a href="/archive" class:active={pathname == '/archive'}>
		<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 16 16"
			><path
				fill="currentColor"
				fill-rule="evenodd"
				d="M3 11.5A1.5 1.5 0 0 0 4.5 13v-2.5a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2V13a1.5 1.5 0 0 0 1.5-1.5V6.036a1 1 0 0 0-.293-.708l-2.035-2.035A1 1 0 0 0 9.964 3H6v1a.5.5 0 0 0 .5.5h3a.75.75 0 0 1 0 1.5h-3a2 2 0 0 1-2-2V3A1.5 1.5 0 0 0 3 4.5zm-1.5 0a3 3 0 0 0 3 3h7a3 3 0 0 0 3-3V6.036a2.5 2.5 0 0 0-.732-1.768l-2.036-2.036A2.5 2.5 0 0 0 9.964 1.5H4.5a3 3 0 0 0-3 3zm8.5-1V13H6v-2.5a.5.5 0 0 1 .5-.5h3a.5.5 0 0 1 .5.5"
				clip-rule="evenodd"
			/></svg
		>
		archive
	</a>

	<div class="spacer"></div>

	<h2>Wallhaven</h2>

	<a href="/wallhaven?sorting=search" class:active={sorting == '?sorting=search'}>
		<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
			><path
				fill="currentColor"
				d="M2 2h20v9h-2V4H4v9.586l5-5l3.914 3.914l-1.414 1.414l-2.5-2.5l-5 5V20h7v2H2zm13.547 5a1 1 0 1 0 0 2a1 1 0 0 0 0-2m-3 1a3 3 0 1 1 6 0a3 3 0 0 1-6 0m5.107 6.5a3.154 3.154 0 1 0 0 6.308a3.154 3.154 0 0 0 0-6.308M12.5 17.654a5.154 5.154 0 1 1 9.437 2.867l1.977 1.98l-1.415 1.413l-1.976-1.978a5.154 5.154 0 0 1-8.023-4.282"
			/></svg
		>
		search</a
	>

	<a href="/wallhaven?sorting=hot" class:active={sorting == '?sorting=hot'}>
		<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 16 16"
			><path
				fill="currentColor"
				d="M8 16c3.314 0 6-2 6-5.5c0-1.5-.5-4-2.5-6c.25 1.5-1.25 2-1.25 2C11 4 9 .5 6 0c.357 2 .5 4-2 6c-1.25 1-2 2.729-2 4.5C2 14 4.686 16 8 16m0-1c-1.657 0-3-1-3-2.75c0-.75.25-2 1.25-3C6.125 10 7 10.5 7 10.5c-.375-1.25.5-3.25 2-3.5c-.179 1-.25 2 1 3c.625.5 1 1.364 1 2.25C11 14 9.657 15 8 15"
			/></svg
		>
		hot</a
	>
	<a href="/wallhaven?sorting=toplist" class:active={sorting == '?sorting=toplist'}>
		<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
			><path
				fill="currentColor"
				d="M13 18v-8l3.5 3.5l1.42-1.42L12 6.16l-5.92 5.92L7.5 13.5L11 10v8zM12 2a10 10 0 0 1 10 10a10 10 0 0 1-10 10A10 10 0 0 1 2 12A10 10 0 0 1 12 2"
			/></svg
		>
		toplist</a
	>
	<a href="/collections" class:active={pathname.startsWith('/collections')}>
		<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
			><path
				fill="currentColor"
				d="M22 16V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2m-11-4l2.03 2.71L16 11l4 5H8M2 6v14a2 2 0 0 0 2 2h14v-2H4V6"
			/></svg
		>
		collections
	</a>

	<div class="spacer"></div>

	<a href="/settings" class:active={pathname == '/settings'}>
		<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
			><path
				fill="currentColor"
				d="M19.5 12c0-.23-.01-.45-.03-.68l1.86-1.41c.4-.3.51-.86.26-1.3l-1.87-3.23a.987.987 0 0 0-1.25-.42l-2.15.91c-.37-.26-.76-.49-1.17-.68l-.29-2.31c-.06-.5-.49-.88-.99-.88h-3.73c-.51 0-.94.38-1 .88l-.29 2.31c-.41.19-.8.42-1.17.68l-2.15-.91c-.46-.2-1-.02-1.25.42L2.41 8.62c-.25.44-.14.99.26 1.3l1.86 1.41a7.3 7.3 0 0 0 0 1.35l-1.86 1.41c-.4.3-.51.86-.26 1.3l1.87 3.23c.25.44.79.62 1.25.42l2.15-.91c.37.26.76.49 1.17.68l.29 2.31c.06.5.49.88.99.88h3.73c.5 0 .93-.38.99-.88l.29-2.31c.41-.19.8-.42 1.17-.68l2.15.91c.46.2 1 .02 1.25-.42l1.87-3.23c.25-.44.14-.99-.26-1.3l-1.86-1.41c.03-.23.04-.45.04-.68m-7.46 3.5c-1.93 0-3.5-1.57-3.5-3.5s1.57-3.5 3.5-3.5s3.5 1.57 3.5 3.5s-1.57 3.5-3.5 3.5"
			/></svg
		>
		settings
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
		font-size: 18px;
		transition:
			background-color 0.3s ease,
			color 0.3s ease;
	}

	.sidebar a svg {
		width: 18px;
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
