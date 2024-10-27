<script>
	import { clients } from '$lib/stores/clients';
	import moment from 'moment';

	let { data } = $props();
	let { settings } = data;
</script>

<div class="clients">
	{#each $clients as client}
		<div class="client">
			<div class="wallpaper">
				{#if client.wallpaper_code}
					<img
						src={`${settings.baseURL}/wallpapers/full/${client.wallpaper_code}`}
						alt={client.wallpaper_code}
					/>
				{:else}
					<img src="/missing.jpeg" />
				{/if}
				{#if client.syncwall}
					<div class="sync-wallpaper">
						<img
							src={`${settings.baseURL}/wallpapers/thumbnail/${client.syncwall}`}
							alt={client.syncwa}
						/>
					</div>
				{/if}
			</div>
			<div class="details">
				{#if client.connected_at}
					<span class="online">Connected {moment(client.connected_at).fromNow()}</span>
				{:else}
					<span class="offline">offline</span>
				{/if}
				<span class="hostname">{client.hostname}</span>
				<span class="addr">{client.addr}</span>
			</div>
		</div>
	{/each}
</div>

<style>
	.client {
		display: flex;
		gap: 30px;
		background-color: rgba(255, 255, 255, 0.1);
		border-radius: 20px;
		padding: 20px;
		margin-inline: 60px;
		margin-bottom: 20px;
	}
	.details {
		display: grid;
		align-items: start;
		justify-content: start;
	}
	.hostname {
		font-size: 28px;
	}
	.wallpaper {
		position: relative;
	}

	.wallpaper img {
		width: 350px;
		overflow: hidden;
		border-radius: 12px;
		aspect-ratio: 16/10;
		object-fit: cover;
	}

	.sync-wallpaper {
		position: absolute;
		bottom: 10px;
		right: 10px;
	}
	.sync-wallpaper img {
		width: 100px;
		aspect-ratio: 16/10;
		object-fit: cover;
		box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.8);
		border: 1px solid black;
		filter: grayscale(80%);
	}
</style>
