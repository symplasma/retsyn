use crate::{
    invocations::invocation::{
        Action, INVOCATION_FILE_PREFIX, Invocation, MAX_INVOCATION_RETENTION, MIN_INVOCATION_NUM,
    },
    retsyn_app::PROJECT_DIRS,
    search_result::SearchResult,
};
use color_eyre::Result;
use std::{
    cmp::Reverse,
    fs::{self, DirEntry, File},
    ops::{Deref, DerefMut},
};
use tracing::warn;

pub(crate) struct InvocationList {
    invocations: Vec<Invocation>,
}

impl Default for InvocationList {
    fn default() -> Self {
        Self {
            invocations: Default::default(),
        }
    }
}

impl Deref for InvocationList {
    type Target = Vec<Invocation>;

    fn deref(&self) -> &Self::Target {
        &self.invocations
    }
}

impl DerefMut for InvocationList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.invocations
    }
}

impl InvocationList {
    pub(crate) fn add_invocation(
        &mut self,
        action: Action,
        query: &str,
        path: &str,
        title: &str,
        url: &str,
    ) {
        self.invocations.push(Invocation::new(
            action,
            query.to_owned(),
            path.to_owned(),
            title.to_owned(),
            url.to_owned(),
        ));
    }

    pub(crate) fn add_invocation_by_item(
        &mut self,
        action: Action,
        query: &str,
        item: &SearchResult,
    ) {
        // url is empty here since this is usually called when we invoke an action on a whole item rather than by clicking on a link
        self.add_invocation(action, query, &item.path, &item.title, "");
    }

    fn cache_files() -> Result<impl Iterator<Item = DirEntry>> {
        let cache_files = PROJECT_DIRS.cache_dir();
        Ok(fs::read_dir(cache_files)?
            .filter_map(Result::ok)
            .filter(|entry| match entry.metadata() {
                Ok(metadata) => {
                    metadata.is_file()
                        && entry
                            .file_name()
                            .to_string_lossy()
                            .starts_with(INVOCATION_FILE_PREFIX)
                }
                Err(_) => false,
            }))
    }

    pub(crate) fn load_from_cache() -> Result<InvocationList> {
        // return all files in an iterator
        let mut cache_files = Self::cache_files()?.collect::<Vec<DirEntry>>();

        // sort the files
        cache_files.sort_by_cached_key(|i| Reverse(i.file_name()));
        let mut cache_file_iter = cache_files.into_iter();

        // take the retention num
        let cache_files_to_load = cache_file_iter.by_ref().take(MAX_INVOCATION_RETENTION);

        let mut invocations = InvocationList::default();

        for cache_file in cache_files_to_load {
            // Create a CSV reader builder
            let mut rdr = csv::Reader::from_reader(File::open(cache_file.path())?);

            // Deserialize each record into our struct
            for result in rdr.deserialize() {
                // TODO might want a custom implementation of deserialize here
                let record: Invocation = result?;
                invocations.push(record);

                // // Ensure that invocations are valid before actually loading them
                // // Only load invocations with IDs
                // // TODO replace with an `is_valid` method or ensure valid invocations on parse
                // if !record.item_id.is_empty() {
                //     match request_sender.send(Box::new(Request::IndexItem {
                //         item: ItemUpdate::Invocation(record),
                //     })) {
                //         Ok(_) => invocation_count += 1,
                //         Err(e) => warn!("could not send invocation: {}", e),
                //     }
                // }
            }

            // exit if we have enough invocations or have read all of the cache files
            // TODO need to bail if we have checked more than MAX_INVOCATION_RETENTION files
            if invocations.len() >= MIN_INVOCATION_NUM {
                break;
            }
        }

        // remove extra cache files
        for file_to_delete in cache_file_iter {
            if let Err(e) = fs::remove_file(file_to_delete.path()) {
                warn!(
                    "Could not remove cache file {}: {}",
                    file_to_delete.path().to_string_lossy(),
                    e
                )
            }
        }

        Ok(invocations)
    }
}

impl<'a> IntoIterator for &'a InvocationList {
    type Item = &'a Invocation;
    type IntoIter = std::slice::Iter<'a, Invocation>;

    fn into_iter(self) -> Self::IntoIter {
        self.invocations.iter()
    }
}
