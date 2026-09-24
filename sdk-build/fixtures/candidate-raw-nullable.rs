use mistralai_candidate_raw_check::generated::types::PromptsUpdateRequest;

#[test]
fn candidate_nullable_request_preserves_omission_null_and_value() {
    let omitted = PromptsUpdateRequest {
        description: None,
        sharing_scope: None,
        title: None,
    };
    assert_eq!(serde_json::to_value(omitted).expect("serialize omission"), serde_json::json!({}));

    let explicit_null = PromptsUpdateRequest {
        description: Some(None),
        sharing_scope: None,
        title: None,
    };
    assert_eq!(
        serde_json::to_value(explicit_null).expect("serialize explicit null"),
        serde_json::json!({"description": null})
    );

    let value = PromptsUpdateRequest {
        description: Some(Some("updated".to_owned())),
        sharing_scope: None,
        title: None,
    };
    assert_eq!(
        serde_json::to_value(value).expect("serialize present value"),
        serde_json::json!({"description": "updated"})
    );
}
