<script>
	import Wallpaper from '../../../lib/components/Wallpaper.svelte';

	let { data } = $props();
	let { collection } = data;
</script>

<div>
	{#each collection as wallpaper}
		<Wallpaper src={wallpaper.thumbs.small} id={wallpaper.id} />
	{/each}
</div>
