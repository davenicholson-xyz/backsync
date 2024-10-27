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
				<img src={`${settings.baseURL}/wallpapers/full/${client.wallpaper_code}`} />
				{#if client.syncwall}
					<div class="sync-wallpaper">
						<img src={`${settings.baseURL}/wallpapers/thumbnail/${client.syncwall}`} />
					</div>
				{/if}
			</div>
		</div>
		<div>
			<span>{client.hostname}</span>
			<span>{client.addr}</span>
			{#if client.connected_at}
				<span>Connected {moment(client.connected_at).fromNow()}</span>
			{/if}
		</div>
	{/each}
</div>

<style>
	.wallpaper {
		width: 450px;
		position: relative;
	}

	.wallpaper img {
		width: 450px;
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
