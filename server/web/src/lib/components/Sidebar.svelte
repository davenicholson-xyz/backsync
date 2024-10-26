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
	<a href="/local" class:active={pathname == '/local'}>
		<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 20 20"
			><path
				fill="currentColor"
				d="M0 2C0 .9.9 0 2 0h14l4 4v14a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2zm5 0v6h10V2zm6 1h3v4h-3z"
			/></svg
		>
		local
	</a>
	<h2>wallhaven</h2>

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
	<a href="/collections" class:active={pathname == '/collections'}>
		<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
			><path
				fill="currentColor"
				d="M22 16V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2m-11-4l2.03 2.71L16 11l4 5H8M2 6v14a2 2 0 0 0 2 2h14v-2H4V6"
			/></svg
		>
		collections
	</a>
</div>

<style>
	.sidebar {
		width: 200px;
		background-color: #2c3e50;
		padding: 20px;
		display: flex;
		flex-direction: column;
		gap: 10px;
		border-radius: 8px;
	}

	.sidebar h2 {
		color: #ecf0f1;
		font-size: 20px;
		margin-top: 15px;
		margin-bottom: 1px;
		padding-left: 5px;
		font-weight: bold;
	}
	.sidebar a {
		display: flex;
		align-items: center;
		gap: 8px;
		text-decoration: none;
		color: #ecf0f1;
		padding: 10px 15px;
		border-radius: 5px;
		font-size: 18px;
		transition:
			background-color 0.3s ease,
			color 0.3s ease;
		font-weight: bold;
	}

	.sidebar a svg {
		width: 26px;
		margin-right: 10px;
	}

	.sidebar a:hover {
		background-color: #34495e;
		color: #f39c12;
	}

	.sidebar a.active {
		background-color: #f39c12;
		color: #2c3e50;
	}
</style>
