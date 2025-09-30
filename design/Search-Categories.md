# Search

## Indexing

- Index all markdown files
- Index projects to 2 levels by default (maybe dependent on temperature)
  - Markdown
  - HTML
  - Plain Text
  - PDF?
  - CSV/TSV?
- Auto indexing on first search, probably in a separate thread
- Reindex in the background on search, update the search on every checkpoint but don't lose selected/highlighted items
  - If items have been deleted since last index, keep them visible but display them as missing
- Incremental re-indexing based on timestamps or checksums
- Filesystem event based re-indexing in background daemon

## Search Interface

- structured output for non-interactive
- TUI (ratatui)
- GUI (eframe)
- Show matches in a snippet
- Cluster results based on file/project
- Sort based on user ranking algorithm (provide a sane default based on match quality and frecency)
- Allow actions on selected matches (default is open)
- Advanced queries based on field specifiers

## Common Data

All search entries should have at least the following fields:

- Title/Name
- Description/Summary
- Kind
- Source: The indexer that found this file
- Path on disk
- Url

### Additional Data

- Hashes?
  - Cryptographic
  - Fuzzy/perceptual
  - CTPH: [ffuzzy](https://lib.rs/crates/ffuzzy) ssdeep Context Triggered Piecewise Hashes
- Vector? If we add embeddings and vector search.

## Categories

- Browser
  - bookmarks
  - history
  - open tabs
  - tab exports and archives (Saved Links)
- Archived Webpages
- Markdown files
- Archived videos
  - transcripts for videos
  - auto-generated descriptions if those can be obtained
- Email
- Music
- Git Repos
- Projects
  - project names
  - readme files
  - project documentation
  - source code?
- Chat history
  - Keybase
  - Signal
  - SimpleX
  - SMS
  - etc.
- KeePassXC Metadata?
