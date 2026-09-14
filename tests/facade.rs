use mistralai::{ChatRequest, Message, Mistral, OcrRequest};

#[test]
fn third_resource_uses_the_same_generated_client_taxonomy() {
    let client = Mistral::new("test");
    let models = client.models();
    let _list_all = models.list();
    let _list_filtered =
        models.list_with(mistralai::models::ListModelsRequest::default().provider("mistral"));
}

#[test]
fn generated_chat_request_maps_to_the_raw_wire_contract() {
    let raw = ChatRequest::new(
        "mistral-small-latest",
        [Message::system("Be concise."), Message::user("Bonjour")],
    )
    .max_tokens(64)
    .temperature(0.2)
    .prompt_cache_key("conversation-1")
    .into_raw();

    let value = serde_json::to_value(raw).unwrap();
    assert_eq!(value["model"], "mistral-small-latest");
    assert_eq!(value["messages"][0]["role"], "system");
    assert_eq!(value["messages"][1]["content"], "Bonjour");
    assert_eq!(value["max_tokens"], 64);
    assert_eq!(value["temperature"], 0.2);
    assert_eq!(value["prompt_cache_key"], "conversation-1");
}

#[test]
fn generated_ocr_request_covers_every_document_input_kind() {
    let document =
        OcrRequest::document_url("mistral-ocr-latest", "https://example.com/document.pdf")
            .include_blocks(true)
            .into_raw();
    let value = serde_json::to_value(document).unwrap();
    assert_eq!(value["document"]["type"], "document_url");
    assert_eq!(value["include_blocks"], true);

    let image = OcrRequest::image_url("ocr", "https://example.com/image.png").into_raw();
    assert!(matches!(
        image.document,
        mistralai::raw::types::OCRRequestDocument::ImageURLChunk(_)
    ));

    let file = OcrRequest::file_id("ocr", uuid::Uuid::nil()).into_raw();
    assert!(matches!(
        file.document,
        mistralai::raw::types::OCRRequestDocument::FileChunk(_)
    ));
}
