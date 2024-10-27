<script>
	import { onDestroy, onMount } from 'svelte';
	import Wallpaper from './Wallpaper.svelte';
	import ScrollToTop from './ScrollToTop.svelte';

	let { nextPage, total_pages } = $props();

	let wallpapers = $state([]);
	let page = $state(1);
	let is_loading = $state(false);

	let observer;

	async function loadPage() {
		if (is_loading || page > total_pages) return;
		is_loading = true;
		const result = await nextPage(page);
		wallpapers = [...wallpapers, ...result];
		page++;
		is_loading = false;
		checkScrollTargetVisibility();
	}

	async function observeElement(entries) {
		const [entry] = entries;
		if (entry.isIntersecting) {
			await loadPage();
		} else if (page >= total_pages) {
			observer.disconnect();
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

	onMount(async () => {
		observer = new IntersectionObserver(observeElement, { rootMargin: '300px' });
		const target = document.querySelector('#scroll-target');

		if (target) {
			observer.observe(target);
		}
	});

	async function checkScrollTargetVisibility() {
		const target = document.querySelector('#scroll-target');
		if (target && isVisibleInViewport(target) && page <= total_pages) {
			await loadPage();
		}
	}

	onDestroy(() => {
		if (observer) observer.disconnect();
	});
</script>

<div class="gallery">
	{#each wallpapers as wallpaper}
		<Wallpaper
			src={wallpaper.thumbnail}
			code={wallpaper.code}
			path={wallpaper.path}
			link={wallpaper.link}
			local={wallpaper.local}
		/>
	{/each}
	<div id="scroll-target"></div>
</div>

<div id="drag-thumbnail">
	<img id="drag-thumbnail-image" data-name="dragthumb" alt="dragged" />
</div>

<ScrollToTop />

<style>
	.gallery {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 22px;
	}

	#scroll-target {
		color: red;
		height: 1px;
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
