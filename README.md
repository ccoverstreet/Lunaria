# Lunaria

This project is designed to be a learning experiment for myself using Rust and Svelte and how integration works. Lunaria is a minimal expense tracking application designed to make recording expenses as easy as possible. Expense items have a variety of identifiers through either a name or the tags attribute. Flexible tags makes filtering and sorting expenses easy.

## Future Plans

- Editable table entries
  	- Should've already done this
- Plot view
  	- Provided an advanced view of entries using something like Plotly
- Clean up error handling with database
  	- Some of the code is not very Rusty
  	- I am still unsure of some design patterns when it comes to propagating errors
  	  	- Need to look into deriving errors
- Accounts/login
- Optimize database
	- Currently just a hashmap of DatabaseEntry structures that is dumped to a JSON file
	- I like rolling my own code, so I might look into designing my own database/file structure
