<script>
	import LocalGallery from '$lib/components/LocalGallery.svelte';

	let { data } = $props();
	let { settings } = data;
</script>

<LocalGallery {settings} />
