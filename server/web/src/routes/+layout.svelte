<script>
	import ClientBar from '../lib/components/ClientBar.svelte';
	let { children } = $props();
</script>

<div>
	<header>
		<ClientBar />
	</header>

	<div class="container">
		<article>
			{@render children()}
		</article>

		<footer>
			<p>footer</p>
		</footer>
	</div>
</div>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Afacad+Flux:wght@100..1000&display=swap');

	:global(body) {
		margin: 0;
		background: black;
		color: white;
		font-family: 'Afacad Flux', sans-serif;
	}

	.container {
		display: grid;
		grid-template-columns: 1fr;
		grid-template-rows: auto 1fr auto;
		grid-template-areas: 'header' 'content' 'footer';
		max-width: 1400px;
		margin-inline: auto;
		min-height: 100vh;
	}

	header {
		grid-area: header;
		margin-block: 0;
		position: sticky;
		top: 0;
		z-index: 1000;
	}

	article {
		grid-area: content;
		overflow: auto;
	}

	footer {
		grid-area: footer;
	}
</style>
