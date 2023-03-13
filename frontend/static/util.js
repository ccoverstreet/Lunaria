

function JSONRequest(url, body) {
	return fetch(url, {
		method: "POST",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(body)
	})
		.then(res => {
			if (res.status < 200 || res.status >= 400)	
				throw res;

			return res;
		})
		.then(res => { return res.json(); })
}

function getById(id) {
	return JSONRequest("/getById", {id: "id1"})
		.then(res => { return res.json() })
}

function debugDisp() {
	const elem = document.querySelector("#debug");

	getById("id1")
		.then(data => {
			elem.value = JSON.stringify(data, null, "   ");
		})
		.catch(err => { elem.value = err } );
}
