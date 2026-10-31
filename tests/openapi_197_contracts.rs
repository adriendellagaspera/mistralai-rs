//! Wire regressions for recursive filters and Responses events from OpenAPI #197.

use mistralai_sdk::api::types::{AndMatchesItem, IpaPhoneme, ResponsesStreamFrameData};
use serde_json::{from_value, json, to_value, Value};

#[test]
fn recursive_filter_unions_preserve_nested_discriminators() {
    let cases = [
        json!({"type": "and", "matches": []}),
        json!({"type": "or", "matches": [
            {"type": "not", "match": {"type": "and", "matches": []}}
        ]}),
        json!({"type": "not", "match": {"type": "or", "matches": []}}),
    ];
    for input in cases {
        let parsed: AndMatchesItem =
            from_value(input.clone()).expect("known recursive filter variant must parse");
        let wire = to_value(&parsed).expect("recursive filter must serialize");
        assert_eq!(wire, input, "known filter discriminators must be lossless");
        let second: AndMatchesItem = from_value(wire).expect("roundtrip must parse");
        assert_eq!(second, parsed);
    }
}

#[test]
fn unknown_recursive_filter_variant_remains_inspectable() {
    let input = json!({"type": "future-filter", "wire": {"enabled": true}});
    let parsed: AndMatchesItem =
        from_value(input.clone()).expect("unknown filters are preserved, not discarded");
    assert_eq!(to_value(parsed).unwrap(), input);
}

#[test]
fn responses_error_event_preserves_wire_discriminator() {
    let input: Value = json!({
        "type": "error",
        "message": "bad request",
        "sequence_number": 1
    });
    let parsed: ResponsesStreamFrameData =
        from_value(input.clone()).expect("Responses error is a typed event");
    let wire = to_value(parsed).expect("typed Responses error must serialize");
    assert_eq!(wire, input);
}

#[test]
fn responses_unknown_event_remains_inspectable() {
    let input: Value = json!({"type": "response.future", "foo": 17});
    let parsed: ResponsesStreamFrameData =
        from_value(input.clone()).expect("unknown Responses event preserved");
    assert_eq!(to_value(parsed).unwrap(), input);
}

#[test]
fn ipa_phonemes_are_typed_without_changing_unicode_wire_values() {
    let cases = [
        (IpaPhoneme::A, "a"),
        (IpaPhoneme::IpaU0061U026A, "aɪ"),
        (IpaPhoneme::IpaU0074U0283, "tʃ"),
        (IpaPhoneme::IpaU03B8, "θ"),
    ];
    for (phoneme, wire) in cases {
        assert_eq!(to_value(&phoneme).unwrap(), json!(wire));
        let restored: IpaPhoneme = from_value(json!(wire)).unwrap();
        assert_eq!(restored, phoneme);
    }
}
