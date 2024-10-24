<script>
	let { data } = $props();
	let { collections } = data;
</script>

<div>
	<h2>collections</h2>
	{#each collections as { label, id }}
		<div>
			<a href={`/collections/${id}`}>{label}</a>
		</div>
	{/each}
</div>
