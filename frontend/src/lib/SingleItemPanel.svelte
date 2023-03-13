<script>
	import ItemDisplay from "$lib/ItemDisplay.svelte"

	let curData = null;
	let testId = "id";

	function getItem() {
		JSONRequest("/api/getById", {
				id: testId
			})
				.then(data => { curData = data; })
				.catch(err => {
					console.error(err);
					curData = null;
				});
	}
</script>

<div id="content">
	<h1>Single Item Display</h1>
	<br>
	<form on:submit={getItem}>
		<label>Get Item By ID</label>
		<input class="input" bind:value={testId}/>
	</form>

	<ItemDisplay bind:data="{curData}"/>
</div>

<style>
	#content {
		width: 100%;
	}
	
	ItemDisplat
</style>
