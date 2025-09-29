# retsyn

Search with retsyn.

## Background

Retsyn is a redo of the indexing, search, and ranking libraries from [educe](https://github.com/symplasma/educe), the extraction, normalization, and data cleaning component of the [Symplasma](https://www.symplasma.com/) project.

It's being rewritten for two primary reasons:

1. Composability: Symplasma is a suite of reference software for an alternate knowledge management, sharing and preservation protocol. In that spirit we're trying to break up the early monolithic code into a set of separate crates that can be used as independent programs or integrated into other software as libraries.
2. Simplicity: Educe was written when I was just getting started with Rust. As such I was still fighting the borrow checker and trying to understand traits. Looking back at the original code it now looks overly complicated, even though it did work. Also [Tantivy](https://github.com/quickwit-oss/tantivy) has been updated and works a bit differently now so we need to re-integrate that.
