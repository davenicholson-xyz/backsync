<script>
	let { label, value, group = $bindable(), klass } = $props();
</script>

<input type="radio" id={value} {value} bind:group />
<label for={value} class={klass}>{label}</label>

<style>
	input[type='radio'] {
		display: none;
	}

	label {
		display: inline-block;
		padding: 6px 18px;
		color: rgba(255, 255, 255, 0.6);
		user-select: none;
		cursor: pointer;
		font-size: 14px;
		background-color: rgba(255, 255, 255, 0.4);
		text-shadow: 0px 2px 2px rgba(0, 0, 0, 0.5);
		color: rgba(255, 255, 255, 0.6);
	}

	label.first {
		border-radius: 4px 0 0 4px;
	}

	label.last {
		border-radius: 0 4px 4px 0;
	}

	input[type='radio']:checked + label {
		background-color: var(--color);
		color: white;
		color: var(--text-color);
		opacity: 1;
	}
</style>
