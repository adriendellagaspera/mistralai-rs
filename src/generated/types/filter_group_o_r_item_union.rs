#[derive(Debug, Clone)]
pub enum FilterGroupORItemUnion {
    FilterGroup(Box<FilterGroup>),
    FilterCondition(FilterCondition),
}
