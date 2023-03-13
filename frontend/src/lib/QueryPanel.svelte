<script>
	import ItemDisplay from "$lib/ItemDisplay.svelte"


	let items = []
	$: netExpense = items.length > 0 ? items.reduce((psum, a) => psum + a.data.val, 0).toFixed(2) : 0.00;
	$: avg = items.length > 0 ? (netExpense / items.length).toFixed(2) : 0.00;
	$: variance = items.length > 0.00 ? 
		(items.reduce((psum, a) => psum + Math.pow(a.data.val - avg, 2), 0) / items.length).toFixed(2) : 0.0;
	$: stddev = Math.sqrt(variance).toFixed(2);

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
				items = data.sort((b, a) => { return date_to_float(a.data.date) - date_to_float(b.data.date)});
				console.log(data);
			});
	}

	export let refreshCallback = () => {
		console.log("ASDAS");
		getItemsAdvanced();
	}
</script>

<div id="controls">
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

	<div id="basic-stats" class="content">
		<h2>Stats</h2>
		<table id="basic-stats-table" class="table">
			<thead>
				<tr>
					<th>Measure</th>
					<th>Value</th>
				</tr>
			</thead>

			<tbody>
				<tr>
					<td>Net</td>
					<td>${netExpense}</td>
				</tr>
				<tr>
					<td>Average</td>
					<td>${avg}</td>
				</tr>
				<tr>
					<td>Variance</td>
					<td>$<sup>2</sup> {variance}</td>
				</tr>
				<tr>
					<td>Std. dev.</td>
					<td>${stddev}</td>
				</tr>
			</tbody>
		</table>
	</div>
</div>

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
	#controls {
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
		justify-content: center;
		gap: 1rem;
	}

	#search {
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
		max-width: 35rem;
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

	#basic-stats {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		padding: 1rem;
	}

	#basic-stats > h2 {
		display: flex;
		justify-content: center;
		padding: 0rem 2rem;
		align-items: center;
	}

	#basic-stats-table {
		min-width: 15rem;
		max-width: 30rem;
	}

	th {
		align-items: center;
		text-align: center;
	}

	td {
		align-items: center;
		text-align: center;
	}
</style>
