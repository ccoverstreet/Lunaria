<script>
	export let data = undefined;
	export let updateCallback = undefined

	let curDate = data.data.date;
	let curName = data.data.name;
	let curVal = data.data.val;
	let curTags = data.data.tags.join(" ");


	function deleteItem() {
		JSONRequest("/api/deleteEntry", {id: data.id})
			.then(res => {
				console.log(res);
				if (updateCallback) updateCallback();
			})
	}

	function updateHandler(event) {
		console.log("Initial", data);
		if (event.key !== "Enter" || !event.shiftKey) return;
		event.preventDefault();

		console.log(curDate);
		console.log(curTags);
		console.log(curVal);
		console.log(curName);

		let newData = {
			id: data.id,
			data: {
				date: curDate,
				name: curName,
				tags: curTags.split(/[ ,\n]+/).filter(x => x.length > 0 ),
				val: parseFloat(curVal),
			}
		}

		console.log(newData);

		JSONRequest("/api/updateEntry", newData)
		if (updateCallback) updateCallback();
	}
</script>


<tr id="wrapper" on:keydown={updateHandler}>

	{#if data === null}
		<td>No item found</td>
	{:else}	
		<!--<p style="width: 40ch;">{data.id}</p>-->
		<td>
			<input class="input" bind:value={curDate}/></td>
		<td><textarea class="textarea" bind:value={curName} rows="2"/></td>
		<td><input class="input" bind:value={curVal}/></td>
		<td id="tags" 
			style="white-space: pre-wrap">
			<textarea class="textarea" bind:value={curTags} rows="2"/>
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

	td  {
		align-items: center;
		text-align: center;
		padding: 0.25rem;
	}

	td > * {
		vertical-align: middle;
		align-items: center;
		text-align: center;
		border: none;
		border-width: 0;
		box-shadow: none;
	}


</style>
