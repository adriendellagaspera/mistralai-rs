use mistralai::{
    ChatClassificationParams, ChatModerationParams, ChatRequest, ClassificationParams,
    ClassifierConversation, EmbeddingParams, FimRequest, Message, Mistral, OcrRequest,
    UpdateModelRequest,
};

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

#[test]
fn expanded_resources_follow_the_official_sdk_taxonomy() {
    let client = Mistral::new("test");

    let models = client.models();
    let _retrieve = models.retrieve("mistral-small-latest");
    let _delete = models.delete("ft:test");
    let _archive = models.archive("ft:test");
    let _unarchive = models.unarchive("ft:test");
    let _update = models.update(
        "ft:test",
        UpdateModelRequest::new()
            .name("renamed")
            .description("description"),
    );

    let embeddings = client.embeddings();
    let _create = embeddings.create(EmbeddingParams::new("mistral-embed", "hello"));

    let fim = client.fim();
    let request = FimRequest::new("codestral-latest", "fn answer() -> ")
        .stop(vec!["\n".to_owned(), "}".to_owned()]);
    let _complete = fim.complete(request.clone());
    let _stream = fim.stream(request);

    let classifiers = client.classifiers();
    let _moderate = classifiers.moderate(ClassificationParams::new(
        "mistral-moderation-latest",
        "safe text",
    ));
    let _classify = classifiers.classify(ClassificationParams::new(
        "classifier-model",
        vec!["first".to_owned(), "second".to_owned()],
    ));
    let _moderate_chat = classifiers.moderate_chat(ChatModerationParams::new(
        "mistral-moderation-latest",
        vec![Message::user("hello")],
    ));
    let conversation = ClassifierConversation::new([Message::user("classify this")]);
    let _classify_chat = classifiers.classify_chat(ChatClassificationParams::new(
        "classifier-model",
        conversation,
    ));

    let files = client.files();
    let _list = files.list();
    let _filtered = files.list_with(
        mistralai::files::ListFilesRequest::default()
            .purpose("batch")
            .page_size(20),
    );
    let _retrieve = files.retrieve("00000000-0000-0000-0000-000000000000");
    let _delete = files.delete("00000000-0000-0000-0000-000000000000");
    let _signed_url = files.get_signed_url(
        mistralai::files::GetSignedUrlFilesRequest::new(
            "00000000-0000-0000-0000-000000000000",
        )
        .expiry(3600),
    );
}

#[test]
fn expanded_request_types_serialize_to_the_openapi_contract() {
    let embedding = serde_json::to_value(
        EmbeddingParams::new("mistral-embed", vec!["one".to_owned(), "two".to_owned()])
            .output_dimension(256)
            .into_raw(),
    )
    .unwrap();
    assert_eq!(embedding["model"], "mistral-embed");
    assert_eq!(embedding["input"], serde_json::json!(["one", "two"]));
    assert_eq!(embedding["output_dimension"], 256);

    let moderation = serde_json::to_value(
        ChatModerationParams::new(
            "mistral-moderation-latest",
            vec![Message::system("policy"), Message::user("content")],
        )
        .into_raw(),
    )
    .unwrap();
    assert_eq!(moderation["input"][0]["role"], "system");
    assert_eq!(moderation["input"][1]["content"], "content");

    let update =
        serde_json::to_value(UpdateModelRequest::new().name("new name").into_raw()).unwrap();
    assert_eq!(update, serde_json::json!({"name": "new name"}));
}
