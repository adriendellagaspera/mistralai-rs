#[derive(Debug, Clone)]
pub enum FilterPayloadFilters {
    FilterGroup(Box<FilterGroup>),
    FilterCondition(FilterCondition),
}
