<script>
	let { label, checked = $bindable() } = $props();
</script>

<label class="checkbox-button">
	<input type="checkbox" bind:checked />
	<span>{label}</span>
</label>

<style>
	.checkbox-button input[type='checkbox'] {
		display: none;
	}

	.checkbox-button span {
		display: inline-block;
		padding: 6px 8px;
		color: white;
		user-select: none;
		cursor: pointer;
		font-size: 14px;
		background-color: rgba(255, 255, 255, 0.6);
		opacity: 0.8;
	}

	.checkbox-button input[type='checkbox']:checked + span {
		background-color: var(--color);
		opacity: 1;
	}
</style>
