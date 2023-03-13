<script>
	let newTags = "";
	let newTitle = "New item";
	let newValue = 0.0;
	let newDate = "2021-01-01";

	let wrapperElem = undefined;

	export let updateCallback = undefined;

	export let showPrompt = () => {
		wrapperElem.style.display = "flex";
	}

	function addItem() {
		let data = {
			name: newTitle,
			val: parseFloat(newValue),
			date: newDate,
			tags: newTags.split(/[ ,]+/)
		};

		console.log(data);

		JSONRequest("/api/addEntry", data)
			.then(res => {
				console.log("Success", res);
				if (updateCallback) updateCallback();
				wrapperElem.style.display = "none";
			})

	}
</script>


<div id="wrapper" bind:this={wrapperElem}>
	<div class="content has-background-light" id="new-item">
		<h2>New Item</h2>
		<form on:submit={()=>{}}>
			<div class="field">
				<label class="label" for="search-tags">Title</label>
				<div class="control">
				<input class="input" id="search-tags" bind:value={newTitle}/>
				</div>
			</div>

			<div class="field">
				<label class="label" for="tags">Tags</label>
				<div class="control">
				<input class="input" id="tags" bind:value={newTags}/>
				</div>
			</div>

			<div class="field">
				<label class="label" for="value">Value</label>
				<div class="control">
				<input class="input" id="value" bind:value={newValue}/>
				</div>
			</div>

			<div class="field">
				<label class="label" for="date">Date</label>
				<div class="control">
				<input class="input" id="date" bind:value={newDate}/>
				</div>
			</div>

			<button on:click={addItem} class="button">Add Item</button>
		</form>
	</div>
</div>

<style>
	#wrapper {
		position: fixed;
		left: 0;
		top: 0;
		z-index: 1000;
		display: flex;
		justify-content: center;
		width: 100%;
	}

	#new-item {
		padding: 1rem;
		margin: auto;
	}
</style>
