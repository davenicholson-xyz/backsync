<script>
	import { settings } from '$lib/stores/settings';
	let { code } = $props();
</script>

<img
	class="localthumb"
	src={`${$settings.app_base}/wallpapers/thumbnail/${code}`}
	alt={`thumb-${code}`}
/>
