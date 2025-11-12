# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.7 (2025-11-12)

### Bug Fixes

 - <csr-id-245d9d3434dae17c572b7d9b28a09d48f666b317/> Selection was stuck on firt item

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Selection was stuck on firt item ([`245d9d3`](https://github.com/symplasma/retsyn/commit/245d9d3434dae17c572b7d9b28a09d48f666b317))
</details>

## v0.1.6 (2025-11-12)

<csr-id-3bf0cd7de6ff5d39760e240e24b134dbfe8e27ff/>
<csr-id-5ba9a8ff4d6c0113d87775b0160d62c1ed36b026/>
<csr-id-4f3e6ee03288165e6b67d5e0e9c6508e1689de35/>
<csr-id-b5bdbe9f75d17aa2b70e1974100eaafdc8f2870f/>

### Chore

 - <csr-id-3bf0cd7de6ff5d39760e240e24b134dbfe8e27ff/> Update features and todos
 - <csr-id-5ba9a8ff4d6c0113d87775b0160d62c1ed36b026/> Upgrade dependencies

### New Features

 - <csr-id-e263804b63faa8c10443ad2f5a4aefb6dfbca26c/> Add indexing status progress bar
 - <csr-id-0bb06a37da8e7ce2687a5018a613c0a82b8e9747/> Enable word stemming for search
 - <csr-id-4a8518f67427471fe4179ac67df4f231d64c3ecf/> Show the preview by default

### Refactor

 - <csr-id-4f3e6ee03288165e6b67d5e0e9c6508e1689de35/> Clean up the update method
 - <csr-id-b5bdbe9f75d17aa2b70e1974100eaafdc8f2870f/> Clean up SearchIndex constructor

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 11 calendar days.
 - 11 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release retsyn v0.1.6 ([`85f3e45`](https://github.com/symplasma/retsyn/commit/85f3e45941fd64f07f8306cc23983933356390a5))
    - Update UI update logic ([`139d032`](https://github.com/symplasma/retsyn/commit/139d032438ea2a5548ac121d52295b323274e1a2))
    - Clean up the update method ([`4f3e6ee`](https://github.com/symplasma/retsyn/commit/4f3e6ee03288165e6b67d5e0e9c6508e1689de35))
    - Collect all results on retrieve_results ([`cea50bd`](https://github.com/symplasma/retsyn/commit/cea50bdf6d17092ab50106b91f1d39747b790a1b))
    - Save egui context to allow update requests ([`b827d57`](https://github.com/symplasma/retsyn/commit/b827d57924d3a4a5967de309bf465b9839a8cd67))
    - Add indexing status progress bar ([`e263804`](https://github.com/symplasma/retsyn/commit/e263804b63faa8c10443ad2f5a4aefb6dfbca26c))
    - Add file indexing counters ([`6606009`](https://github.com/symplasma/retsyn/commit/66060091aba0d78029f2ecc22262912a2decb81e))
    - Enable word stemming for search ([`0bb06a3`](https://github.com/symplasma/retsyn/commit/0bb06a37da8e7ce2687a5018a613c0a82b8e9747))
    - Clean up SearchIndex constructor ([`b5bdbe9`](https://github.com/symplasma/retsyn/commit/b5bdbe9f75d17aa2b70e1974100eaafdc8f2870f))
    - Show the preview by default ([`4a8518f`](https://github.com/symplasma/retsyn/commit/4a8518f67427471fe4179ac67df4f231d64c3ecf))
    - Update features and todos ([`3bf0cd7`](https://github.com/symplasma/retsyn/commit/3bf0cd7de6ff5d39760e240e24b134dbfe8e27ff))
    - Add request ids and other optimizations ([`28af738`](https://github.com/symplasma/retsyn/commit/28af738f92cb71d582d592d984c88549125f3ddd))
    - Initial attempt at background updates ([`0958c4a`](https://github.com/symplasma/retsyn/commit/0958c4a27bde2982d5d00303778475ef88abc27a))
    - Upgrade dependencies ([`5ba9a8f`](https://github.com/symplasma/retsyn/commit/5ba9a8ff4d6c0113d87775b0160d62c1ed36b026))
</details>

## v0.1.5 (2025-10-31)

<csr-id-bb98f4425945fb432c5db8c467a2cdb17fa6ea59/>

### Chore

 - <csr-id-bb98f4425945fb432c5db8c467a2cdb17fa6ea59/> Enable clippy print lints and switch to tracing

### New Features

 - <csr-id-c0f9f3a2d1664bccd001aa37a11af591f299a1d8/> implement Display trait for IndexPath enum
 - <csr-id-0bb5c89287050af725daa074bb8f35684ee299d4/> add Display trait placeholder comment for IndexPath enum
 - <csr-id-8ff99f7d8b27b5090975e6fc6c7cb6031e81ba66/> Add tracing to a rotating log file
 - <csr-id-0fb60865ea9dc6a61e91cafa970f9514ce6541de/> Add AichatSessionFiles collector
 - <csr-id-a06369fee536b56f48720cfd8ed2e3f342c8ccc9/> Run articles through arc90 Readability
 - <csr-id-37fbe8d28632f2e237db557897b402877d902a5f/> Add indexing of WebScrapbook Archives

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 6 calendar days.
 - 8 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release retsyn v0.1.5 ([`f583868`](https://github.com/symplasma/retsyn/commit/f583868721ba83edbe043ca2cc93ebdb0709ed9e))
    - Add rust crates to design docs ([`36a1a30`](https://github.com/symplasma/retsyn/commit/36a1a30faf5863db82220f09bfcd02ec30892aa7))
    - Enable clippy print lints and switch to tracing ([`bb98f44`](https://github.com/symplasma/retsyn/commit/bb98f4425945fb432c5db8c467a2cdb17fa6ea59))
    - Implement Display trait for IndexPath enum ([`c0f9f3a`](https://github.com/symplasma/retsyn/commit/c0f9f3a2d1664bccd001aa37a11af591f299a1d8))
    - Add Display trait placeholder comment for IndexPath enum ([`0bb5c89`](https://github.com/symplasma/retsyn/commit/0bb5c89287050af725daa074bb8f35684ee299d4))
    - Add tracing to a rotating log file ([`8ff99f7`](https://github.com/symplasma/retsyn/commit/8ff99f7d8b27b5090975e6fc6c7cb6031e81ba66))
    - Add features ([`8e66261`](https://github.com/symplasma/retsyn/commit/8e6626111e7fe1b2f05caf8392b36447919743e7))
    - Add AichatSessionFiles collector ([`0fb6086`](https://github.com/symplasma/retsyn/commit/0fb60865ea9dc6a61e91cafa970f9514ce6541de))
    - Run articles through arc90 Readability ([`a06369f`](https://github.com/symplasma/retsyn/commit/a06369fee536b56f48720cfd8ed2e3f342c8ccc9))
    - Add indexing of WebScrapbook Archives ([`37fbe8d`](https://github.com/symplasma/retsyn/commit/37fbe8d28632f2e237db557897b402877d902a5f))
    - Add WebScrapbookFiles collector ([`efc4d00`](https://github.com/symplasma/retsyn/commit/efc4d0017ddf1bb4d53a3daaa15932660e6f9581))
    - Add semantic search crates ([`2924616`](https://github.com/symplasma/retsyn/commit/292461662082b657529bfefa718f93ae8441c9c8))
</details>

## v0.1.4 (2025-10-23)

<csr-id-8e0b24605debea5acd07494882ec9e2b820ef577/>
<csr-id-da581c19ff4cb413cfb370219080d89cb4e890ce/>

### New Features

 - <csr-id-4a2d90c4f6ad3c789e727641c9549b59aaea7ab5/> Add result limit to UI
 - <csr-id-45ce9f9104133309e7cef3a3f06db02b5b9dee80/> Add query conjunction toggle
 - <csr-id-27efbc051747cfb50cd367511ae77d41580620ae/> implement single-click selection and double-click activation for search results

### Bug Fixes

 - <csr-id-222b4dbcb3900964076be14d06b4d53bb9c09bb0/> Fix Claude's overly complex solution
 - <csr-id-a4584f2572fc470abe9f92a446564343753ec2aa/> Correct config screen layout

### Refactor

 - <csr-id-8e0b24605debea5acd07494882ec9e2b820ef577/> Variablize and increase the result limit
 - <csr-id-da581c19ff4cb413cfb370219080d89cb4e890ce/> Move Cli parsing into separate module

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release retsyn v0.1.4 ([`936309c`](https://github.com/symplasma/retsyn/commit/936309c85d0ba5df2ea45f67213ce55a0b73afa8))
    - Add result limit to UI ([`4a2d90c`](https://github.com/symplasma/retsyn/commit/4a2d90c4f6ad3c789e727641c9549b59aaea7ab5))
    - Variablize and increase the result limit ([`8e0b246`](https://github.com/symplasma/retsyn/commit/8e0b24605debea5acd07494882ec9e2b820ef577))
    - Add query conjunction toggle ([`45ce9f9`](https://github.com/symplasma/retsyn/commit/45ce9f9104133309e7cef3a3f06db02b5b9dee80))
    - Fix Claude's overly complex solution ([`222b4db`](https://github.com/symplasma/retsyn/commit/222b4dbcb3900964076be14d06b4d53bb9c09bb0))
    - Implement single-click selection and double-click activation for search results ([`27efbc0`](https://github.com/symplasma/retsyn/commit/27efbc051747cfb50cd367511ae77d41580620ae))
    - Add config save todos ([`afec609`](https://github.com/symplasma/retsyn/commit/afec60985e8dc12f6d25a026d988986e7a3991da))
    - Correct config screen layout ([`a4584f2`](https://github.com/symplasma/retsyn/commit/a4584f2572fc470abe9f92a446564343753ec2aa))
    - Move Cli parsing into separate module ([`da581c1`](https://github.com/symplasma/retsyn/commit/da581c19ff4cb413cfb370219080d89cb4e890ce))
    - Update features ([`4309340`](https://github.com/symplasma/retsyn/commit/43093403bba4dc349a684ace291780595ef06215))
</details>

## v0.1.3 (2025-10-21)

<csr-id-23293c7da5d078ed1d5fcebd3f4519b7e2a70fbc/>
<csr-id-850f1acd09171676a4a6ab38cab567f7c23bf997/>
<csr-id-e2e73ef454e1fb2b9b873134f4ea9d11fe5a60e1/>

### Chore

 - <csr-id-23293c7da5d078ed1d5fcebd3f4519b7e2a70fbc/> Clean up tantivy field handling
 - <csr-id-850f1acd09171676a4a6ab38cab567f7c23bf997/> Improve help screen

### New Features

 - <csr-id-3a4696bd6641e33d2aef7fa80b04215ba0c7b1a8/> Add support for fuzzy search
 - <csr-id-2f503e2854c6b365b3bf735ebf76e6793f271016/> implement markdown title extraction with frontmatter, h1, and filename fallback
 - <csr-id-7382a33a041b56eac9a2f44a054ec82d47933fe5/> Add toggle for query leniency
 - <csr-id-07f1635b2f61f2bd29effe34ba25e9b23b72bf9c/> Add Ctrl+P shortcut to toggle preview pane and update help screen

### Refactor

 - <csr-id-e2e73ef454e1fb2b9b873134f4ea9d11fe5a60e1/> Switch to columns_const

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release retsyn v0.1.3 ([`d75d4e7`](https://github.com/symplasma/retsyn/commit/d75d4e755a1f90335e30784e7c4435f874b0a58c))
    - Add support for fuzzy search ([`3a4696b`](https://github.com/symplasma/retsyn/commit/3a4696bd6641e33d2aef7fa80b04215ba0c7b1a8))
    - Switch to columns_const ([`e2e73ef`](https://github.com/symplasma/retsyn/commit/e2e73ef454e1fb2b9b873134f4ea9d11fe5a60e1))
    - Clean up tantivy field handling ([`23293c7`](https://github.com/symplasma/retsyn/commit/23293c7da5d078ed1d5fcebd3f4519b7e2a70fbc))
    - Implement markdown title extraction with frontmatter, h1, and filename fallback ([`2f503e2`](https://github.com/symplasma/retsyn/commit/2f503e2854c6b365b3bf735ebf76e6793f271016))
    - Improve help screen ([`850f1ac`](https://github.com/symplasma/retsyn/commit/850f1acd09171676a4a6ab38cab567f7c23bf997))
    - Add toggle for query leniency ([`7382a33`](https://github.com/symplasma/retsyn/commit/7382a33a041b56eac9a2f44a054ec82d47933fe5))
    - Add Ctrl+P shortcut to toggle preview pane and update help screen ([`07f1635`](https://github.com/symplasma/retsyn/commit/07f1635b2f61f2bd29effe34ba25e9b23b72bf9c))
</details>

## v0.1.2 (2025-10-20)

### New Features

 - <csr-id-397e5944a502448568652382717c0376f34f38a2/> Enable tooltips for links in preview
 - <csr-id-9ca505ee0deb2f50d0a7d80d61efc8d0206ee732/> Improve markdown rendering in preview
 - <csr-id-1dc7f6686f342c5d0186beeea53177f8d85beddc/> Add document preview

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release retsyn v0.1.2 ([`49ff2d9`](https://github.com/symplasma/retsyn/commit/49ff2d947cb9bdf205096e8f55333e6f3e49202f))
    - Enable tooltips for links in preview ([`397e594`](https://github.com/symplasma/retsyn/commit/397e5944a502448568652382717c0376f34f38a2))
    - Improve markdown rendering in preview ([`9ca505e`](https://github.com/symplasma/retsyn/commit/9ca505ee0deb2f50d0a7d80d61efc8d0206ee732))
    - Add document preview ([`1dc7f66`](https://github.com/symplasma/retsyn/commit/1dc7f6686f342c5d0186beeea53177f8d85beddc))
    - Add draw_preview_area method ([`06325b8`](https://github.com/symplasma/retsyn/commit/06325b8c41be7e37f7d180576a183297a47e995f))
</details>

## v0.1.1 (2025-10-20)

### New Features

 - <csr-id-e34e0842636c8b0416ba56d1581fa24c8d4c0837/> add configuration screen with file picker and first-run detection
 - <csr-id-6d0d7ef496b1b045695474e5be62415a8affb662/> add help screen with keyboard shortcuts and search syntax
 - <csr-id-64617991273b7a80beaefd72b5d04a2a926c2212/> Threaded search is working
 - <csr-id-51fa03df43cb23654a726526d300d38337d708df/> add IndexEntry trait with core methods for indexing
 - <csr-id-02eba6adb00294631616893c26fa4ca4275eb663/> add index_entry module to src directory
 - <csr-id-e3d194e4e5be15f8aa70e4ee4b5090240f42b575/> add toggle functionality for Snippets button
 - <csr-id-221ed5f0652b03e3b28f04d83b81f0ec92687209/> add snippets button toggle functionality in UI
 - <csr-id-2dac19cdc365b946453068a089b4c215cb26e06c/> Add button bar
 - <csr-id-41c58514e1917630a69976e29b9a9ae19b0e4f86/> Make headers easier to read
 - <csr-id-6a325471a5e538439c8e9b1870321f0806b49f69/> add full-width snippet rendering with `add_sized`
 - <csr-id-e34ed9d14999b05cb7f1ad9b4b40dd351f7d8202/> add comment for full-width widget placement in search result UI
 - <csr-id-f5839c083cf353c562477c036109fd2ec61e2ffd/> Add CLI flags for default config and index clearing

### Bug Fixes

 - <csr-id-5d2d0f8cb6f7ba147fec82c9e614e678704ce8b2/> Add missing dependency
 - <csr-id-cb44951db356e43202b0b2e9253db6eb3d28c9c8/> Display results and preview as columns
 - <csr-id-648b8729984609e59c76dca8e23d224be39cb31a/> Fix broken code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 21 commits contributed to the release over the course of 3 calendar days.
 - 4 days passed between releases.
 - 15 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release retsyn v0.1.1 ([`769ee0b`](https://github.com/symplasma/retsyn/commit/769ee0b5ac54915cebcaaa8a1e58728b18c57d29))
    - Add missing dependency ([`5d2d0f8`](https://github.com/symplasma/retsyn/commit/5d2d0f8cb6f7ba147fec82c9e614e678704ce8b2))
    - Add configuration screen with file picker and first-run detection ([`e34e084`](https://github.com/symplasma/retsyn/commit/e34e0842636c8b0416ba56d1581fa24c8d4c0837))
    - Add help screen with keyboard shortcuts and search syntax ([`6d0d7ef`](https://github.com/symplasma/retsyn/commit/6d0d7ef496b1b045695474e5be62415a8affb662))
    - Threaded search is working ([`6461799`](https://github.com/symplasma/retsyn/commit/64617991273b7a80beaefd72b5d04a2a926c2212))
    - First attempt at threaded updates ([`3c40ec8`](https://github.com/symplasma/retsyn/commit/3c40ec8d390fb7be8a2317b9e57abff74940b5fe))
    - Add IndexEntry trait with core methods for indexing ([`51fa03d`](https://github.com/symplasma/retsyn/commit/51fa03df43cb23654a726526d300d38337d708df))
    - Add index_entry module to src directory ([`02eba6a`](https://github.com/symplasma/retsyn/commit/02eba6adb00294631616893c26fa4ca4275eb663))
    - Add update_entry function ([`3119398`](https://github.com/symplasma/retsyn/commit/3119398edf6383339d4366a3d49436d2a523810d))
    - Display results and preview as columns ([`cb44951`](https://github.com/symplasma/retsyn/commit/cb44951db356e43202b0b2e9253db6eb3d28c9c8))
    - First attempt at preview area ([`ea91a94`](https://github.com/symplasma/retsyn/commit/ea91a94149e66e6b66abd3ce284871df7a1324a4))
    - Add toggle functionality for Snippets button ([`e3d194e`](https://github.com/symplasma/retsyn/commit/e3d194e4e5be15f8aa70e4ee4b5090240f42b575))
    - Add snippets button toggle functionality in UI ([`221ed5f`](https://github.com/symplasma/retsyn/commit/221ed5f0652b03e3b28f04d83b81f0ec92687209))
    - Add button bar ([`2dac19c`](https://github.com/symplasma/retsyn/commit/2dac19cdc365b946453068a089b4c215cb26e06c))
    - Make headers easier to read ([`41c5851`](https://github.com/symplasma/retsyn/commit/41c58514e1917630a69976e29b9a9ae19b0e4f86))
    - Make snippets full-width ([`a924b28`](https://github.com/symplasma/retsyn/commit/a924b2887f23dc5518d461acabe78e977b94ecf7))
    - Add full-width snippet rendering with `add_sized` ([`6a32547`](https://github.com/symplasma/retsyn/commit/6a325471a5e538439c8e9b1870321f0806b49f69))
    - Add comment for full-width widget placement in search result UI ([`e34ed9d`](https://github.com/symplasma/retsyn/commit/e34ed9d14999b05cb7f1ad9b4b40dd351f7d8202))
    - Fix broken code ([`648b872`](https://github.com/symplasma/retsyn/commit/648b8729984609e59c76dca8e23d224be39cb31a))
    - Add CLI flags for default config and index clearing ([`f5839c0`](https://github.com/symplasma/retsyn/commit/f5839c083cf353c562477c036109fd2ec61e2ffd))
    - Reorganize features ([`bb2a404`](https://github.com/symplasma/retsyn/commit/bb2a404a82f1949393dd225f98513832014dc312))
</details>

## v0.1.0 (2025-10-16)

<csr-id-008a3c24b0d906daa743d26cd9de8f57a0375a9e/>
<csr-id-3a19f067ad3403a14b9a0160c572fa71324f1db1/>
<csr-id-faa9ac25570629142e2f43ef5f32a5474cde5d98/>
<csr-id-118b653150f2b3a712c8622fce84c9e2efafc1c0/>
<csr-id-fbb32f53eb8ddf668f9be93143c13d601364058c/>
<csr-id-abb3c1cd9f232f4a018b8e273477843c718366e0/>
<csr-id-d3645e1695b5d060521aaffe10c7b1cede52b501/>

### New Features

 - <csr-id-28c2d70032dc5ec26b4dfc82b215d54d44a97d52/> Hold Alt to open/reveal without quit
 - <csr-id-01bb248203176c8d34455aa62f6cdf8d8d23279d/> Get open and reveal working
 - <csr-id-b140ea8d9d1c125e4932198bd384bd77da93c35d/> Add incremental updates
 - <csr-id-ef08ea09e05d11d732fba83cf12a83312949b9a8/> Delete files from the index that need updates
 - <csr-id-5a98211cc08061a9ae0e344f7182ccd98902e61d/> Read last indexing epoch
 - <csr-id-3220659bf73baaac19f56bbde7dfce35550ba2c5/> Cache last indexing time
 - <csr-id-93f400ff89faedcdf45a2263ff72707054f57769/> Add source and indexed_at fields
 - <csr-id-7f44b462d5f65fe7e306fe6d0206060240112954/> Add path to search results
 - <csr-id-f4c97d88fd5dec07be0897dd21d38c15a2b8036b/> Add path and field constants
 - <csr-id-b05321e0367276b6967df1b420991a489589a00e/> Show query parsing errors
 - <csr-id-2c5b37802c367d8c3907d269062b426415a80e92/> Add snippets to search results
 - <csr-id-6c917f93ac7dbf5a48fd86eb6f102c73562f04f3/> Add search functionality
 - <csr-id-8bdf5b05dea0e15a9a0271f328ccea1ed7973bbc/> Add tantivy indexing
 - <csr-id-797c2252aa0280e91d6bfd72768be91bbc2ade2f/> Allow clearing search
 - <csr-id-75ba6c5bfbb687d269ebbb6f6a88dd74476e8ced/> Default to light mode
 - <csr-id-d88ba05b7f268cdee41ddbf52396b811ca287ab0/> create initial Rust search app with eframe and egui

### Bug Fixes

 - <csr-id-1acd7d9a4dca8ddf47c8355a03a1be72e8e69c15/> Fix incremental document detection
 - <csr-id-ba961e990c02e018b0e02afc20dac82a9c83d3d0/> resolve borrow conflict in search item click handling

### Performance

 - <csr-id-4b02b951faf48d1f08c4adf7b203a596c3a2b795/> Only check file modified time for existing

### Refactor

 - <csr-id-008a3c24b0d906daa743d26cd9de8f57a0375a9e/> Add methods for incremental updates
 - <csr-id-3a19f067ad3403a14b9a0160c572fa71324f1db1/> Split out item drawing
 - <csr-id-faa9ac25570629142e2f43ef5f32a5474cde5d98/> Add RetsynApp struct
 - <csr-id-118b653150f2b3a712c8622fce84c9e2efafc1c0/> Add draw_main_ui
 - <csr-id-fbb32f53eb8ddf668f9be93143c13d601364058c/> Add handle_key_events
 - <csr-id-abb3c1cd9f232f4a018b8e273477843c718366e0/> Remove `#[derive(Default)]` from `SearchApp` struct

### Style

 - <csr-id-d3645e1695b5d060521aaffe10c7b1cede52b501/> add light grey background to search result snippets

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 51 commits contributed to the release over the course of 17 calendar days.
 - 26 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release retsyn v0.1.0 ([`5541493`](https://github.com/symplasma/retsyn/commit/55414933249a5db957e89756b10d7c26c20a48d3))
    - Add auto-generated changelog ([`620f97f`](https://github.com/symplasma/retsyn/commit/620f97f8e37fe4849587f91230b8d2d609361987))
    - Add missing cargo metadata ([`b8fa75d`](https://github.com/symplasma/retsyn/commit/b8fa75d1ffe46a697d7debe8ce13dc6b59418e2e))
    - Add license and status ([`f5e3ddd`](https://github.com/symplasma/retsyn/commit/f5e3ddd181862816e132ed30fc60a09ca7376dd7))
    - Add more features ([`71c8ee7`](https://github.com/symplasma/retsyn/commit/71c8ee7f2ea0dac7e1212e280ab20afbdb43bc53))
    - Update features ([`1d823cc`](https://github.com/symplasma/retsyn/commit/1d823cc53631e9270233a0a61fd8f7d500d97314))
    - Hold Alt to open/reveal without quit ([`28c2d70`](https://github.com/symplasma/retsyn/commit/28c2d70032dc5ec26b4dfc82b215d54d44a97d52))
    - Get open and reveal working ([`01bb248`](https://github.com/symplasma/retsyn/commit/01bb248203176c8d34455aa62f6cdf8d8d23279d))
    - Fix incremental document detection ([`1acd7d9`](https://github.com/symplasma/retsyn/commit/1acd7d9a4dca8ddf47c8355a03a1be72e8e69c15))
    - Add methods for incremental updates ([`008a3c2`](https://github.com/symplasma/retsyn/commit/008a3c24b0d906daa743d26cd9de8f57a0375a9e))
    - Refactor and add logging messages ([`d454553`](https://github.com/symplasma/retsyn/commit/d454553b8fb8fa1ca2043b1d591576e85cd3f251))
    - Only check file modified time for existing ([`4b02b95`](https://github.com/symplasma/retsyn/commit/4b02b951faf48d1f08c4adf7b203a596c3a2b795))
    - Add incremental updates ([`b140ea8`](https://github.com/symplasma/retsyn/commit/b140ea8d9d1c125e4932198bd384bd77da93c35d))
    - Delete files from the index that need updates ([`ef08ea0`](https://github.com/symplasma/retsyn/commit/ef08ea09e05d11d732fba83cf12a83312949b9a8))
    - Read last indexing epoch ([`5a98211`](https://github.com/symplasma/retsyn/commit/5a98211cc08061a9ae0e344f7182ccd98902e61d))
    - Cache last indexing time ([`3220659`](https://github.com/symplasma/retsyn/commit/3220659bf73baaac19f56bbde7dfce35550ba2c5))
    - Add source and indexed_at fields ([`93f400f`](https://github.com/symplasma/retsyn/commit/93f400ff89faedcdf45a2263ff72707054f57769))
    - Add path to search results ([`7f44b46`](https://github.com/symplasma/retsyn/commit/7f44b462d5f65fe7e306fe6d0206060240112954))
    - Add todo about spacing ([`0b79ce8`](https://github.com/symplasma/retsyn/commit/0b79ce8b4b1d340293b9c17f26e289f97ff29833))
    - Add path and field constants ([`f4c97d8`](https://github.com/symplasma/retsyn/commit/f4c97d88fd5dec07be0897dd21d38c15a2b8036b))
    - Show query parsing errors ([`b05321e`](https://github.com/symplasma/retsyn/commit/b05321e0367276b6967df1b420991a489589a00e))
    - Add light grey background to search result snippets ([`d3645e1`](https://github.com/symplasma/retsyn/commit/d3645e1695b5d060521aaffe10c7b1cede52b501))
    - Add snippets to search results ([`2c5b378`](https://github.com/symplasma/retsyn/commit/2c5b37802c367d8c3907d269062b426415a80e92))
    - Split out item drawing ([`3a19f06`](https://github.com/symplasma/retsyn/commit/3a19f067ad3403a14b9a0160c572fa71324f1db1))
    - Add rust crates ([`d2f986a`](https://github.com/symplasma/retsyn/commit/d2f986a3d4a9797b69c47fe97c4ad71624d00956))
    - Add Features and TODOs section ([`d51dd16`](https://github.com/symplasma/retsyn/commit/d51dd16b325cf4aacdaf82dbbe44bc36e2c7d1e9))
    - Add search functionality ([`6c917f9`](https://github.com/symplasma/retsyn/commit/6c917f93ac7dbf5a48fd86eb6f102c73562f04f3))
    - Add tantivy indexing ([`8bdf5b0`](https://github.com/symplasma/retsyn/commit/8bdf5b05dea0e15a9a0271f328ccea1ed7973bbc))
    - Add tilde expansion and traversal ([`707a06b`](https://github.com/symplasma/retsyn/commit/707a06ba4fa60bf83aeff6d45b56124cbb0f369d))
    - Add config file ([`485974e`](https://github.com/symplasma/retsyn/commit/485974e921dd9d0ff249a4917a11794d93990918))
    - Add RetsynApp struct ([`faa9ac2`](https://github.com/symplasma/retsyn/commit/faa9ac25570629142e2f43ef5f32a5474cde5d98))
    - Change search categories to sources ([`a79f26d`](https://github.com/symplasma/retsyn/commit/a79f26df96c1fdf48756ad98e26a2102c657d510))
    - Allow clearing search ([`797c225`](https://github.com/symplasma/retsyn/commit/797c2252aa0280e91d6bfd72768be91bbc2ade2f))
    - Add draw_main_ui ([`118b653`](https://github.com/symplasma/retsyn/commit/118b653150f2b3a712c8622fce84c9e2efafc1c0))
    - Add handle_key_events ([`fbb32f5`](https://github.com/symplasma/retsyn/commit/fbb32f53eb8ddf668f9be93143c13d601364058c))
    - Add additional categories ([`1b187a1`](https://github.com/symplasma/retsyn/commit/1b187a104ea85441cf5231ec121025d7ce649883))
    - Add common and additional data ([`64f6c04`](https://github.com/symplasma/retsyn/commit/64f6c046d16cc72532deb969a9fe6ca14290d0fc))
    - Default to light mode ([`75ba6c5`](https://github.com/symplasma/retsyn/commit/75ba6c5bfbb687d269ebbb6f6a88dd74476e8ced))
    - Add nix shell file ([`8f6badb`](https://github.com/symplasma/retsyn/commit/8f6badb770c36ddaddabf93f657da82cb2e2afdb))
    - Add rust toolchain file ([`f34c127`](https://github.com/symplasma/retsyn/commit/f34c127d1054c9adfca7d5b84dc9a1c5dd0d7b5f))
    - Add nix shell section ([`4d3309e`](https://github.com/symplasma/retsyn/commit/4d3309ed129fa5163863cf967b86659225aae034))
    - Update Rust edition ([`b9d423c`](https://github.com/symplasma/retsyn/commit/b9d423c81531419f5d546e9d6950dceb362a7df5))
    - Update eframe and egui versions ([`9256e5b`](https://github.com/symplasma/retsyn/commit/9256e5b389960c1a94270ebe2b7d86436b8956ef))
    - Resolve borrow conflict in search item click handling ([`ba961e9`](https://github.com/symplasma/retsyn/commit/ba961e990c02e018b0e02afc20dac82a9c83d3d0))
    - Remove `#[derive(Default)]` from `SearchApp` struct ([`abb3c1c`](https://github.com/symplasma/retsyn/commit/abb3c1cd9f232f4a018b8e273477843c718366e0))
    - Add Cargo.lock and ignore target ([`f8b83cd`](https://github.com/symplasma/retsyn/commit/f8b83cd20239bef1134bed45c4cb04b7edc62d3d))
    - Create initial Rust search app with eframe and egui ([`d88ba05`](https://github.com/symplasma/retsyn/commit/d88ba05b7f268cdee41ddbf52396b811ca287ab0))
    - Add Aider ignores ([`20fff1e`](https://github.com/symplasma/retsyn/commit/20fff1e3b87443b7cc3efbaea8a67e5e78042f9f))
    - Add basic features design doc ([`af8937c`](https://github.com/symplasma/retsyn/commit/af8937c40f4ccca88e2bdc1ba63d48b6274685ab))
    - Add design documents ([`e21a753`](https://github.com/symplasma/retsyn/commit/e21a7531c97621368d4d32159e3b6806657adf90))
    - Initial Commit ([`3b26be1`](https://github.com/symplasma/retsyn/commit/3b26be118637c1c710c58f72f201d52d6ed565d8))
</details>

