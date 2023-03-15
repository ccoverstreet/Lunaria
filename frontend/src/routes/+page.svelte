<script>
	import QueryPanel from "$lib/QueryPanel.svelte";
	import AddItem from "$lib/AddItem.svelte";
	import { onMount } from "svelte";
	import GithubLogo from "$lib/assets/github-mark-white.svg"

	let addItemElem;
	let showAddElemPrompt;

	let queryPanelRefresh = undefined;

	let refreshView = () => {
		if (queryPanelRefresh) queryPanelRefresh()
	}
	
	function addItem() {
		console.log(queryPanelRefresh);
		showAddElemPrompt();
	}
</script>

<div id="toolbar" class="has-background-info-dark">
	<button class="button is-primary" on:click={addItem}>Add</button>
	<div style="flex-grow: 1"></div>
	<h2 class="has-text-primary-light">Lunaria</h2>
	<div style="flex-grow: 1"></div>
	<a style="height: 100%" href="https://github.com/ccoverstreet/Lunaria">
		<img src={GithubLogo} style="height: 100%; padding: 0.25rem;" alt="GitHub logo"/>
	</a>
</div>

<div id="content">
	<AddItem bind:this={addItemElem} bind:showPrompt={showAddElemPrompt}
		 	 bind:updateCallback={refreshView}/>
	<QueryPanel bind:refreshCallback={queryPanelRefresh}/>
</div>

<style>
	#toolbar {
		position: fixed;
		top: 0;
		left: 0;
		height: 3rem;
		display: flex;
		align-items: center;
		padding: 0 0.5rem;
		width: 100%;
		z-index: 100;
	}

	#toolbar > h2 {
		font-weight: bold;
		font-size: 1.5rem;
	}

	#content {
		margin-top: 3rem;
	}

</style>





