use crate::{
    invocations::invocation::{Action, Invocation},
    search_result::SearchResult,
};

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

    pub(crate) fn append(&mut self, invocations: &mut InvocationList) {
        self.invocations.append(&mut invocations.invocations);
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
}

impl<'a> IntoIterator for &'a InvocationList {
    type Item = &'a Invocation;
    type IntoIter = std::slice::Iter<'a, Invocation>;

    fn into_iter(self) -> Self::IntoIter {
        self.invocations.iter()
    }
}
