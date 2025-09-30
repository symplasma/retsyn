# Symplasma Search Architecture

## Indexing Phases

### Enumerate

- Find all of the files to index
  - Directories to search
  - Filetypes to index vs ignore
  - Recursion Depth
- Determine which files need to be updated
- Allow the forcing of a full re-indexing

### Clean/Transform

- Run filters to convert files into indexable data
  - Convert HTML to Markdown to avoid indexing HTML tags and attributes with the content. Do we ever want to be able to search the HTML attributes and tags?
  - OCR images to make text searchable. Anything better than tesseract?
  - Describe images with something like CLIP but more modern and better. Inspired by [SmartScan](https://f-droid.org/en/packages/com.fpf.smartscan/) ([source](https://github.com/dev-diaries41/smartscan)): SmartScan, app that automatically organizes your images by content similarity and enables text-based media search.
  - Create embeddings via something like fastembed-rs

### Index

- Read through files and separate URLs and the rest of the text
- URLs need to be added to the link index
- Add data to indexes
  - Fulltext index (tantivy)
  - Link database
  - Graph database (something like Terminus or dGraph)
  - Vector database for embeddings based search
  - Soundex or other phoenetic indexing?

## Search Phases

### Search Providers Incrementally

- Allow a user configurable order of search providers. They should be run concurrently, but in phases:
  - Local data
  - Our remote nodes
  - Remote nodes of friends (how do we handle recursion)
  - Broader internet providers e.g. DuckDuckGo, HN, etc.
- Providers should return a result with an excerpt showing where the match occurs
- Excerpts should probably be lazy loaded

### Result Ranking

- Collet and rank results according to a user defined algorithm

### Filtering and Display

- Allow toggling between different levels of detail e.g.
  - whole projects vs matching files within projects
  - Links vs all of the various places where a given, normalized link shows up

## Components

- Search providers
- Ranking algorithms
