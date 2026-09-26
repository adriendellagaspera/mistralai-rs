use mistralai::raw::types::{
    ChatCompletionRequest, ClassificationRequestInput, EmbeddingRequestInput,
};
use mistralai::{
    ClassifyClassifiersRequest, CompleteChatRequest, CompleteFimRequest, CreateEmbeddingsRequest,
    Mistral, ModerateClassifiersRequest, UpdateModelsRequest,
};

#[test]
fn resources_follow_the_canonical_288_taxonomy() {
    let client = Mistral::new("test");

    let models = client.models();
    let _list_all = models.list();
    let _list_filtered =
        models.list_with(mistralai::models::ListModelsRequest::default().provider("mistral"));
    let _update = models.update(
        "ft:test",
        UpdateModelsRequest::new()
            .name("renamed")
            .description("description"),
    );

    let _ocr = client.ocr();
    let _files = client.files();
    let _classifiers = client.classifiers();
    let _embeddings = client.embeddings();
    let _fim = client.fim();
    let _chat = client.chat();
}

#[test]
fn canonical_chat_request_maps_to_the_raw_wire_contract() {
    let raw: ChatCompletionRequest = serde_json::from_value(serde_json::json!({
        "model": "mistral-small-latest",
        "messages": [
            {"role": "system", "content": "Be concise."},
            {"role": "user", "content": "Bonjour"}
        ],
        "max_tokens": 64,
        "temperature": 0.2,
        "prompt_cache_key": "conversation-1"
    }))
    .unwrap();

    let value = serde_json::to_value(CompleteChatRequest::from(raw).into_raw()).unwrap();
    assert_eq!(value["model"], "mistral-small-latest");
    assert_eq!(value["messages"][0]["role"], "system");
    assert_eq!(value["messages"][1]["content"], "Bonjour");
    assert_eq!(value["max_tokens"], 64);
    assert_eq!(value["temperature"], 0.2);
    assert_eq!(value["prompt_cache_key"], "conversation-1");
}

#[test]
fn canonical_request_builders_serialize_to_the_openapi_contract() {
    let embedding_input: EmbeddingRequestInput =
        serde_json::from_value(serde_json::json!("hello")).unwrap();
    let embedding = serde_json::to_value(
        CreateEmbeddingsRequest::new(embedding_input, "mistral-embed")
            .output_dimension(256)
            .into_raw(),
    )
    .unwrap();
    assert_eq!(embedding["model"], "mistral-embed");
    assert_eq!(embedding["input"], "hello");
    assert_eq!(embedding["output_dimension"], 256);

    let classification_input: ClassificationRequestInput =
        serde_json::from_value(serde_json::json!("safe text")).unwrap();
    let classify =
        ClassifyClassifiersRequest::new(classification_input.clone(), "classifier-model");
    let moderate =
        ModerateClassifiersRequest::new(classification_input, "mistral-moderation-latest");
    assert_eq!(
        serde_json::to_value(classify.into_raw()).unwrap()["model"],
        "classifier-model"
    );
    assert_eq!(
        serde_json::to_value(moderate.into_raw()).unwrap()["model"],
        "mistral-moderation-latest"
    );

    let fim = CompleteFimRequest::new("fn answer() -> ", "codestral-latest")
        .max_tokens(64)
        .into_raw();
    assert_eq!(fim.model, "codestral-latest");
    assert_eq!(fim.prompt, "fn answer() -> ");
    assert_eq!(fim.max_tokens, Some(Some(64)));

    let update =
        serde_json::to_value(UpdateModelsRequest::new().name("new name").into_raw()).unwrap();
    assert_eq!(update, serde_json::json!({"name": "new name"}));
}
