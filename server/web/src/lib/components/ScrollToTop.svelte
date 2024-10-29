<script>
	import Icon from '@iconify/svelte';
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
	<Icon icon="tdesign:align-top" />
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
		padding: 8px 16px;
		border-radius: 10px;
		background: black;
		opacity: 0.6;
		font-size: 40px;
	}

	.scroll-to-top:hover {
		opacity: 1;
	}

	.hidden {
		opacity: 0;
		pointer-events: none;
	}
</style>
