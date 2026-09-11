#[derive(Debug, Clone)]
pub enum FilterGroupANDItemUnion {
    FilterGroup(Box<FilterGroup>),
    FilterCondition(FilterCondition),
}
