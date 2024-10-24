<script>
	import { onDestroy, onMount } from 'svelte';
	import Wallpaper from './Wallpaper.svelte';

	let { url, settings } = $props();

	let wallpapers = $state([]);
	let page = $state(1);
	let total_pages = $state(1);
	let is_loading = $state(false);

	let observer;

	async function loadPage() {
		if (is_loading) return;
		if (page > total_pages) return;

		is_loading = true;

		let p_url = `${url}&page=${page}`;
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
			wallpapers = [...wallpapers, ...data.data];
			total_pages = data.meta.last_page;
			page++;
		} catch (e) {
			console.error('API', e);
		} finally {
			is_loading = false;
		}
	}

	function observeElement(entries) {
		const [entry] = entries;
		if (entry.isIntersecting) {
			loadPage();
		}
	}

	const isVisibleInViewport = (element) => {
		const rect = element.getBoundingClientRect();
		return (
			rect.top >= 0 &&
			rect.left >= 0 &&
			rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) &&
			rect.right <= (window.innerWidth || document.documentElement.clientWidth)
		);
	};

	async function loadUntilNotInView(target) {
		await loadPage();
		if (total_pages > 1) {
			while (isVisibleInViewport(target)) {
				await loadPage();
			}
		}
	}

	onMount(async () => {
		observer = new IntersectionObserver(observeElement, { rootMargin: '200px' });
		const target = document.querySelector('#scroll-target');

		if (target) {
			observer.observe(target);
			loadUntilNotInView(target);
		}
	});

	onDestroy(() => {
		if (observer) observer.disconnect();
	});
</script>

<div class="gallery">
	{#each wallpapers as wallpaper}
		<Wallpaper src={wallpaper.thumbs.small} code={wallpaper.id} />
	{/each}
</div>

<div id="scroll-target"></div>

<div id="drag-thumbnail">
	<img id="drag-thumbnail-image" data-name="dragthumb" />
</div>

<style>
	.gallery {
		display: flex;
		flex-wrap: wrap;
		gap: 18px;
	}

	#scroll-target {
		color: red;
		height: 20px;
	}

	#drag-thumbnail {
		position: absolute;
		top: -100px;
		left: -100px;
	}

	#drag-thumbnail img {
		width: 100px;
		aspect-ratio: 16 / 9;
		object-fit: cover;
		display: none;
	}
</style>
