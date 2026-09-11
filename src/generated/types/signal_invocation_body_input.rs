///Input data for the signal, matching its schema
#[derive(Debug, Clone)]
pub enum SignalInvocationBodyInput {
    NetworkEncodedInput(NetworkEncodedInput),
    SignalInvocationBodyVariant2(SignalInvocationBodyVariant2),
}
