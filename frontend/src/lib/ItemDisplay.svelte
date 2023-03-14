<script>
	export let data = undefined;
	export let updateCallback = undefined

	$: content = JSON.stringify(data, "", "   ");

	let curDate;
	let curName;
	let curVal;
	let curTags;

	function deleteItem() {
		JSONRequest("/api/deleteEntry", {id: data.id})
			.then(res => {
				console.log(res);
				if (updateCallback) updateCallback();
			})
	}

	function updateHandler(event) {
		if (event.key !== "Enter" || !event.shiftKey) return;
		event.preventDefault();

		let newData = {
			id: data.id,
			data: {
				date: curDate,
				name: curName,
				tags: curTags.split(/[ ,\n]+/).filter(x => x.length > 0 ),
				val: parseFloat(curVal),
			}
		}

		console.log(curDate);
		console.log(curTags);
		console.log(curVal);
		console.log(curName);

		console.log(newData);

		JSONRequest("/api/updateEntry", newData)
	}
</script>


<tr id="wrapper" on:keydown={updateHandler}>

	{#if data === null}
		<td>No item found</td>
	{:else}	
		<!--<p style="width: 40ch;">{data.id}</p>-->
		<td contenteditable=true bind:textContent={curDate}>{data.data.date}</td>
		<td contenteditable=true bind:textContent={curName}>{data.data.name}</td>
		<td contenteditable=true bind:textContent={curVal}>{data.data.val.toFixed(2)}</td>
		<td id="tags" contenteditable=true bind:textContent={curTags} 
			style="white-space: pre-wrap">
			{#each data.data.tags as tag}
				<p>{tag} </p>
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
		text-align: center;
	}

	
	#tags {
		flex-direction: row;
		flex-wrap: wrap;
	}

	#tags > * {
		padding: 0.5rem;
	}
</style>
