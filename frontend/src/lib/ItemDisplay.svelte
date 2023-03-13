<script>
	export let data = undefined;
	export let updateCallback = undefined

	$: content = JSON.stringify(data, "", "   ");

	function deleteItem() {
		JSONRequest("/api/deleteEntry", {id: data.id})
			.then(res => {
				console.log(res);
				if (updateCallback) updateCallback();
			})
	}
</script>


<tr id="wrapper">

	{#if data === null}
		<td>No item found</td>
	{:else}	
		<!--<p style="width: 40ch;">{data.id}</p>-->
		<td>{data.data.date}</td>
		<td>{data.data.name}</td>
		<td>{data.data.val}</td>
		<td id="tags">
			{#each data.data.tags as tag}
				<p>{tag}</p>
			{/each}
		</td>
		<td style="flex-grow: 0; width: auto;">
			<button class="button is-danger" on:click={deleteItem}
		  	  style=" padding: 1rem; margin: 0.5rem;">X</button>
		</td>
	{/if}
</tr>


<style>
	#wrapper {
		align-items: center;
		width: 100%;
	}

	td {
		vertical-align: middle;
		align-items: center;
	}

	
	#tags {
		flex-direction: row;
		flex-wrap: wrap;
	}

	#tags > * {
		padding: 0.5rem;
	}
</style>
