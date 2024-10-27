<script>
	let { label, checked = $bindable(), klass } = $props();
</script>

<label class="checkbox-button {klass}">
	<input type="checkbox" bind:checked />
	<span>{label}</span>
</label>

<style>
	.checkbox-button input[type='checkbox'] {
		display: none;
	}

	.checkbox-button span {
		display: inline-block;
		padding: 6px 18px;
		color: rgba(255, 255, 255, 0.6);
		user-select: none;
		cursor: pointer;
		font-size: 14px;
		background-color: rgba(255, 255, 255, 0.4);
		text-shadow: 0px 1px 2px rgba(0, 0, 0, 0.5);
		color: rgba(255, 255, 255, 0.6);
	}

	.checkbox-button.first span {
		border-radius: 4px 0 0 4px;
	}

	.checkbox-button.last span {
		border-radius: 0 4px 4px 0;
	}
	.checkbox-button input[type='checkbox']:checked + span {
		background-color: var(--color);
		color: white;
		color: var(--text-color);
		opacity: 1;
	}
</style>
