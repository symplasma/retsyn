// TODO might want to change this to an enum so we can add an index update event
pub(crate) struct IndexRequest {
    pub(crate) query: String,
    pub(crate) limit: usize,
    pub(crate) lenient: bool,
    pub(crate) query_conjunction: bool,
    pub(crate) fuzziness: u8,
}
