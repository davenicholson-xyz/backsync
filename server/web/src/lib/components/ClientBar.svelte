<script>
	import { clients } from '$lib/stores/clients';
	import Client from '$lib/components/Client.svelte';
</script>

<div class="client-bar">
	{#each $clients as client}
		<Client {client} />
	{/each}
</div>

<style>
	.client-bar {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 20px;
		margin-top: 20px;
		margin-bottom: 30px;
		padding-block: 20px;
		background: rgba(0, 0, 0, 0.9);
	}
</style>
