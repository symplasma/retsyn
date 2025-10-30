# retsyn

Search with retsyn.

## Status: Alpha

This is currently **Alpha stage** software. That means that it is incomplete and mostly untested/unproven.

## Background

Retsyn is a redo of the indexing, search, and ranking libraries from [educe](https://github.com/symplasma/educe), the extraction, normalization, and data cleaning component of the [Symplasma](https://www.symplasma.com/) project.

It's being rewritten for two primary reasons:

1. Composability: Symplasma is a suite of reference software for an alternate knowledge management, sharing and preservation protocol. In that spirit we're trying to break up the early monolithic code into a set of separate crates that can be used as independent programs or integrated into other software as libraries.
2. Simplicity: Educe was written when I was just getting started with Rust. As such I was still fighting the borrow checker and trying to understand traits. Looking back at the original code it now looks overly complicated, even though it did work. Also [Tantivy](https://github.com/quickwit-oss/tantivy) has been updated and works a bit differently now so we need to re-integrate that.

## Features and TODOs

This is a list of features. Implemented features are checked, the rest are planned.

- [x] Ensure that invalid queries display errors gracefully
- [x] Get item opening working
- [x] Hold `Alt` to open or reveal an item without quitting
- [x] Hold `Shift` to reveal an item in the file browser
- [x] Show snippets for items
- [x] Allow updates for items in the full text index
- [x] Allow incremental updates to the index
- [x] Ask the user for config info on first launch (needs testing)
- [x] Configuration screen
- [ ] Add better navigation
  - [x] Clear query via `Ctrl+u` or `Esc`
  - [x] Clear to end of query via `Ctrl+k`
  - [ ] Word forward/back
  - [ ] Beginning/end of query
  - [ ] Clicking on item selects it
  - [ ] Double clicking item launches it
- [x] Add help screen with search syntax guide
- [ ] Ensure that search debounce is working correctly
- [ ] Show all items that fit within the current window
- [ ] Record and show recent queries
  - [ ] Record and show recent item activations
  - [ ] Activations should be associated with the search that found them
  - [ ] We might want the query/activation relation hierarchy to be invertible
- [x] Allow toggling between fuzzy and exact search
- [x] Do indexing in background threads
- [x] Add toggle for display of snippets
- [ ] Do indexing after starting UI interaction
- [ ] Display indexing status in UI
- [ ] Auto-toggle snippets off when fuzzy search is active ([Snippet generation breaks with fuzzy fields · Issue #2576](https://github.com/quickwit-oss/tantivy/issues/2576))
- [ ] Add highlighting of terms in title
- [ ] Add garbage collection for tantivy store after indexing completes
- [x] Allow clearing of search indexes via a CLI flag
- [ ] Add more indexing sources and types
- [ ] Add file type converters
  - [ ] HTML to Markdown so we can search only content
  - [ ] OCR for images
  - [ ] Speech to text for audio
- [ ] Add scoping based on item source
- [x] Ensure that queries allow for powerful searches

### Future Features

- [ ] Add the link database, probably sqlite
- [ ] Add phonetic search e.g. [rphonetic](https://lib.rs/crates/rphonetic): Rust port of phonetic Apache commons-codec algorithms
- [ ] Add vector search based on embeddings e.g.
  - [ck-search](https://lib.rs/crates/ck-search): Semantic grep by embedding - find code by meaning, not just keywords
  - [fastembed](https://lib.rs/crates/fastembed): Library for generating vector embeddings, reranking locally.
- [ ] Merge/deduplicate multiple hits on the same item from different indexes
- [ ] Add item ranking for all sources
- [ ] Add custom item ranking algorithm
- [ ] Add auto re-ranking based on user guidance and examples
