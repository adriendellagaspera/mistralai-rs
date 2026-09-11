///Input data for the query, matching its schema
#[derive(Debug, Clone)]
pub enum QueryInvocationBodyInput {
    NetworkEncodedInput(NetworkEncodedInput),
    QueryInvocationBodyVariant2(QueryInvocationBodyVariant2),
}
