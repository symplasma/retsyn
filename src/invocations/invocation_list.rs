use crate::{invocations::invocation::Invocation, search_result::SearchResult};

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

impl InvocationList {
    pub(crate) fn add_invocation(&mut self, path: &str, title: &str, url: &str) {
        self.invocations.push(Invocation::new(
            path.to_owned(),
            title.to_owned(),
            url.to_owned(),
        ));
    }

    pub(crate) fn append(&mut self, invocations: &mut InvocationList) {
        self.invocations.append(&mut invocations.invocations);
    }

    pub(crate) fn add_invocation_by_item(&mut self, item: &SearchResult) {
        self.add_invocation(&item.path, &item.title, "");
    }
}

impl<'a> IntoIterator for &'a InvocationList {
    type Item = &'a Invocation;
    type IntoIter = std::slice::Iter<'a, Invocation>;

    fn into_iter(self) -> Self::IntoIter {
        self.invocations.iter()
    }
}
