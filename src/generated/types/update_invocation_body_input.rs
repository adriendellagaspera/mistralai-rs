///Input data for the update, matching its schema
#[derive(Debug, Clone)]
pub enum UpdateInvocationBodyInput {
    NetworkEncodedInput(NetworkEncodedInput),
    UpdateInvocationBodyVariant2(UpdateInvocationBodyVariant2),
}
