<script>
	import Gallery from './Gallery.svelte';

	let { settings } = $props();
	let total_pages = $state(1);

	$inspect(total_pages);

	async function nextPage(p) {
		try {
			let response = await fetch(`${settings.baseURL}/wallpapers/page/${p}`);
			let data = await response.json();

			if (data.error) {
				throw new Error(data.error);
			}

			let wallpaper_data = data.wallpapers.map((w) => ({
				path: w.origin,
				code: w.code,
				thumbnail: `${settings.baseURL}/wallpapers/thumbnail/${w.code}`,
				local: true
			}));

			total_pages = data.pages;

			return wallpaper_data;
		} catch (e) {
			console.error(e);
		}
	}
</script>

<Gallery {nextPage} {total_pages} />
