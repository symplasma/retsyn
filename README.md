# retsyn

Search with retsyn.

## Background

Retsyn is a redo of the indexing, search, and ranking libraries from [educe](https://github.com/symplasma/educe), the extraction, normalization, and data cleaning component of the [Symplasma](https://www.symplasma.com/) project.

It's being rewritten for two primary reasons:

1. Composability: Symplasma is a suite of reference software for an alternate knowledge management, sharing and preservation protocol. In that spirit we're trying to break up the early monolithic code into a set of separate crates that can be used as independent programs or integrated into other software as libraries.
2. Simplicity: Educe was written when I was just getting started with Rust. As such I was still fighting the borrow checker and trying to understand traits. Looking back at the original code it now looks overly complicated, even though it did work. Also [Tantivy](https://github.com/quickwit-oss/tantivy) has been updated and works a bit differently now so we need to re-integrate that.

## Features and TODOs

This is a list of features. Implemented features are checked, the rest are planned.

- [x] Ensure that invalid queries display errors gracefully
- [ ] Allow clearing of search indexes via a CLI flag
- [x] Get item opening working
- [x] Hold `Alt` to open or reveal an item without quitting
- [x] Hold `Shift` to reveal an item in the file browser
- [ ] Show all items that fit within the current window
- [x] Show snippets for items
- [ ] Record and show recent queries
- [x] Allow updates for items in the full text index
- [x] Allow incremental updates to the index
- [ ] Ensure that search debounce is working correctly
- [ ] Allow toggling between fuzzy and exact search
- [ ] Allow indexing in another thread
- [ ] Do indexing in background thread
- [ ] Add garbage collection for tantivy store after indexing completes

### Future Features

- [ ] Add the link database
- [ ] Add phonetic search
- [ ] Add vector search based on embeddings
- [ ] Merge/deduplicate multiple hits on the same item from different indexes
- [ ] Add scoping based on item source
- [ ] Ensure that queries allow for powerful searches
- [ ] Add item ranking for all sources
- [ ] Add custom item ranking algorithm
- [ ] Add more indexing sources and types
