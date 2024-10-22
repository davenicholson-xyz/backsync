<script>
  export let data
  let { clients } = data
  console.log(clients)
</script>


<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<h1>Clients</h1>
{#each clients as client}
<p>{client.hostname}</p>
{/each}

<style>
h1 {
    font-size: 20px;
    color: red
  }
</style>
