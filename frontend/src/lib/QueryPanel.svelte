<script>
	import ItemDisplay from "$lib/ItemDisplay.svelte"


	let items = []
	$: netExpense = items.length > 0? items.reduce((psum, a) => psum + a.data.val, 0).toFixed(2) : 0;
	$: avg = items.length > 0 ? netExpense / items.length : 0;
	$: variance = items.length > 0 ? 
		items.reduce((psum, a) => psum + Math.pow(a.data.val - avg, 2), 0) / items.length : 0;
	$: stddev = Math.sqrt(variance);

	let searchTags = "";
	let searchTitle = "";
	let searchYear = "";
	let searchMonth = "";
	let searchDay = "";


	function date_to_float(d) {
		const x = d.split("-");

		return x[0] * 366 + x[1] * 30 + x[2] * 1;
	}

	function getItemsByTags() {
		console.log(searchTags.split());
		JSONRequest("/api/getByTags", {
			tags: searchTags.split(/[ ,]+/)
		})	
			.then(data => {
				items = data.sort((a, b) => { return date_to_float(a.data.date) - date_to_float(b.data.date)});
				console.log(data);

			})
	}

	function getItemsByMonth() {
		const split = searchTags.split(" ");
		const year = parseInt(split[0]);
		const month = parseInt(split[1]);
		console.log(year, month);
		JSONRequest("/api/getByMonth", {
			year: year,
			month: month
		})
			.then(data => {
				console.log(data);
			})
	}

	function getItemsAdvanced() {
		const sub = {
			tags: searchTags.split(/[ ,]+/).filter(tag => tag.length > 0),
			title: searchTitle,
			year: parseInt(searchYear),
			month: parseInt(searchMonth),
			day: parseInt(searchDay)
		};

		console.log(sub);

		JSONRequest("/api/getByAdvanced", sub)
			.then(data => {
				items = data.sort((a, b) => { return date_to_float(a.data.date) - date_to_float(b.data.date)});
				console.log(data);
			});
	}

	export let refreshCallback = () => {
		console.log("ASDAS");
		getItemsAdvanced();
	}
</script>

<div id="search">
	<form on:submit={getItemsAdvanced}>
		<div class="field">
			<label class="label" for="search-tags">Title contains</label>
			<div class="control">
			<input class="input" id="search-tags" bind:value={searchTitle}/>
			</div>
		</div>

		<div class="field">
			<label class="label" for="search-tags">Tags</label>
			<div class="control">
			<input class="input" id="search-tags" bind:value={searchTags}/>
			</div>
		</div>


		<div id="time-search">
			<div class="field">
				<label class="label" for="search-tags">Year</label>
				<div class="control">
				<input class="input" id="search-tags" bind:value={searchYear}/>
				</div>
			</div>

			<div class="field">
				<label class="label" for="search-tags">Month</label>
				<div class="control">
				<input class="input" id="search-tags" bind:value={searchMonth}/>
				</div>
			</div>

			<div class="field">
				<label class="label" for="search-tags">Day</label>
				<div class="control">
				<input class="input" id="search-tags" bind:value={searchDay}/>
				</div>
			</div>
		</div>

		<button style="display: none"></button>
	</form>
</div>

<p>Net: ${netExpense}</p>
<p>Average: ${avg}</p>
<p>Variance: ${variance}</p>
<p>Standard dev.: ${stddev}</p>
<table class="table">
	<thead>
		<th>Date</th>
		<th>Name</th>
		<th>Amount</th>
		<th>Tags</th>
		<th>Delete</th>
	</thead>

	<tbody>
		{#each items as d}
			<ItemDisplay data={d} updateCallback={getItemsAdvanced}/>
		{/each}
	</tbody>
</table>

<style>
	#search {
		width: 100%;
		display: flex;
		justify-content: center;
		padding: 1rem;
	}

	#time-search {
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
		gap: 1rem;
		width: 100%;
	}

	#time-search > * {
		flex-grow: 1;
	}

	form {
		min-width: 20rem;
		max-width: 60rem;
		width: 100%;
	}

	table {
		width: 100%;
	}

	#header {
		margin-top: 1rem;
		display: flex;
	}

	#header > * {
		display: flex;
		justify-content: center;
		font-weight: bold;
		width: 25%;
	}
</style>
