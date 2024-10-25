<script>
	import Gallery from './Gallery.svelte';

	let { url, settings } = $props();
	let total_pages = $state(1);

	async function nextPage(p) {
		let p_url = `${url}&page=${p}`;
		try {
			let response = await fetch(`${settings.baseURL}/wallhaven/search`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ url: p_url })
			});
			let data = await response.json();

			if (data.error) {
				throw new Error(data.error);
			}

			let wallhaven_data = data.data.map((w) => ({
				path: w.path,
				code: w.id,
				thumbnail: w.thumbs.small
			}));

			total_pages = data.meta.last_page;

			return wallhaven_data;
		} catch (e) {
			console.error(e);
		}
	}
</script>

<Gallery {nextPage} {total_pages} />
