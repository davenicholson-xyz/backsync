<script>
	import { onMount } from 'svelte';

	let show = $state(false);

	function handleScroll() {
		show = window.scrollY > 100;
	}

	function scrollToTop() {
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}

	onMount(() => {
		window.addEventListener('scroll', handleScroll);
		return () => window.removeEventListener('scroll', handleScroll);
	});
</script>

<div class="scroll-to-top {show ? '' : 'hidden'}" onclick={scrollToTop}>
	<svg xmlns="http://www.w3.org/2000/svg" width="" height="" viewBox="0 0 24 24"
		><path fill="currentColor" d="M5 15h4v6h6v-6h4l-7-8zM4 3h16v2H4z" /></svg
	>
</div>

<style>
	.scroll-to-top {
		display: grid;
		place-items: center;
		position: fixed;
		bottom: 20px;
		right: 50%;
		cursor: pointer;
		transition: opacity 0.3s;
		padding: 8px 6px;
		border-radius: 10px;
		background: black;
		opacity: 0.6;
	}

	.scroll-to-top:hover {
		opacity: 1;
	}

	.scroll-to-top svg {
		width: 35px;
		height: 35px;
	}

	.hidden {
		opacity: 0;
		pointer-events: none;
	}
</style>
