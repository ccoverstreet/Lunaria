<script>
	import QueryPanel from "$lib/QueryPanel.svelte";
	import AddItem from "$lib/AddItem.svelte";
	import { onMount } from "svelte";

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

<button class="button is-primary" on:click={addItem}>Add</button>
<AddItem bind:this={addItemElem} bind:showPrompt={showAddElemPrompt}
		 bind:updateCallback={refreshView}
	style="display: fixed; top: 0; left: 0"/>
<QueryPanel bind:refreshCallback={queryPanelRefresh}/>



