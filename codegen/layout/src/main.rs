use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use syn::{ImplItem, Item, ItemImpl, Type};

const INDEX_SIZE: usize = 200;
const TARGET_RUST_LINES: usize = 1_200;
const MAX_ATOMIC_RUST_LINES: usize = 5_000;

fn main() {
    let generated = env::args()
        .nth(1)
        .expect("usage: mistralai-codegen-layout <generated-dir>");
    let generated = PathBuf::from(generated);
    split_file(&generated.join("types.rs"), "types", false);
    split_file(&generated.join("client.rs"), "client", true);
}

fn split_file(path: &Path, directory: &str, split_http_client: bool) {
    let source = fs::read_to_string(path).expect("read generated Rust");
    let parsed = syn::parse_file(&source).expect("parse generated Rust");
    let mut items = Vec::new();
    for item in parsed.items {
        if split_http_client {
            items.extend(split_http_client_impl(item));
        } else {
            items.push(item);
        }
    }

    let output_dir = path.parent().unwrap().join(directory);
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).expect("remove stale generated layout");
    }
    fs::create_dir_all(&output_dir).expect("create generated layout directory");

    let mut used = BTreeMap::<String, usize>::new();
    let mut files = Vec::new();
    for (index, item) in items.into_iter().enumerate() {
        let base = item_name(&item, index);
        let count = used.entry(base.clone()).or_default();
        *count += 1;
        let filename = if *count == 1 {
            format!("{base}.rs")
        } else {
            format!("{base}_{}.rs", count)
        };
        let file = syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: vec![item],
        };
        write_atomic_rust(&output_dir.join(&filename), prettyplease::unparse(&file));
        files.push(filename);
    }

    let mut index_files = Vec::new();
    for (batch, chunk) in files.chunks(INDEX_SIZE).enumerate() {
        let filename = format!("_index_{batch:03}.rs");
        let body = chunk
            .iter()
            .map(|file| format!("include!(\"{file}\");\n"))
            .collect::<String>();
        write_layout_rust(&output_dir.join(&filename), body);
        index_files.push(filename);
    }

    let mut facade = syn::File {
        shebang: parsed.shebang,
        attrs: parsed.attrs,
        items: Vec::new(),
    };
    for index in index_files {
        let include: Item = syn::parse_str(&format!("include!(\"{directory}/{index}\");"))
            .expect("build include item");
        facade.items.push(include);
    }
    write_layout_rust(path, prettyplease::unparse(&facade));
}

fn write_layout_rust(path: &Path, source: String) {
    let lines = source.lines().count();
    assert!(
        lines <= TARGET_RUST_LINES,
        "generated layout file {} has {lines} lines; maximum is {TARGET_RUST_LINES}",
        path.display()
    );
    fs::write(path, source).expect("write generated layout file");
}

fn write_atomic_rust(path: &Path, source: String) {
    let lines = source.lines().count();
    if lines > TARGET_RUST_LINES {
        eprintln!(
            "warning: generated atomic Rust item {} has {lines} lines; target is {TARGET_RUST_LINES}",
            path.display()
        );
    }
    assert!(
        lines <= MAX_ATOMIC_RUST_LINES,
        "generated atomic Rust item {} has {lines} lines; hard maximum is {MAX_ATOMIC_RUST_LINES}",
        path.display()
    );
    fs::write(path, source).expect("write generated atomic item");
}

fn split_http_client_impl(item: Item) -> Vec<Item> {
    let Item::Impl(item_impl) = item else {
        return vec![item];
    };
    if item_impl.trait_.is_some() || !is_http_client(&item_impl.self_ty) || item_impl.items.len() <= 1 {
        return vec![Item::Impl(item_impl)];
    }
    item_impl
        .items
        .iter()
        .cloned()
        .map(|member| {
            let mut part = item_impl.clone();
            part.items = vec![member];
            Item::Impl(part)
        })
        .collect()
}

fn is_http_client(ty: &Type) -> bool {
    match ty {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "HttpClient"),
        _ => false,
    }
}

fn item_name(item: &Item, index: usize) -> String {
    let raw = match item {
        Item::Const(item) => item.ident.to_string(),
        Item::Enum(item) => item.ident.to_string(),
        Item::Fn(item) => item.sig.ident.to_string(),
        Item::Mod(item) => item.ident.to_string(),
        Item::Static(item) => item.ident.to_string(),
        Item::Struct(item) => item.ident.to_string(),
        Item::Trait(item) => item.ident.to_string(),
        Item::Type(item) => item.ident.to_string(),
        Item::Union(item) => item.ident.to_string(),
        Item::Impl(item) => impl_name(item),
        Item::Use(_) => format!("use_{index:04}"),
        Item::Macro(_) => format!("macro_{index:04}"),
        _ => format!("item_{index:04}"),
    };
    snake_case(&raw)
}

fn impl_name(item: &ItemImpl) -> String {
    let target = match item.self_ty.as_ref() {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
    .unwrap_or_else(|| "type".to_string());
    let member = item.items.first().and_then(|item| match item {
        ImplItem::Fn(method) => Some(method.sig.ident.to_string()),
        ImplItem::Const(value) => Some(value.ident.to_string()),
        ImplItem::Type(value) => Some(value.ident.to_string()),
        _ => None,
    });
    match member {
        Some(member) => format!("impl_{target}_{member}"),
        None => format!("impl_{target}"),
    }
}

fn snake_case(value: &str) -> String {
    let mut out = String::new();
    for (index, ch) in value.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if index > 0 && !out.ends_with('_') {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_matches('_').chars().take(96).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_http_client_methods_into_independent_impls() {
        let item: Item = syn::parse_str(
            "impl HttpClient { pub fn alpha(&self) {} pub async fn beta(&self) {} }",
        )
        .unwrap();
        let parts = split_http_client_impl(item);
        assert_eq!(parts.len(), 2);
        assert_eq!(item_name(&parts[0], 0), "impl_http_client_alpha");
        assert_eq!(item_name(&parts[1], 1), "impl_http_client_beta");
    }

    #[test]
    fn does_not_split_trait_impls() {
        let item: Item = syn::parse_str(
            "impl Default for HttpClient { fn default() -> Self { todo!() } }",
        )
        .unwrap();
        assert_eq!(split_http_client_impl(item).len(), 1);
    }

    #[test]
    fn names_are_deterministic_and_bounded() {
        assert_eq!(snake_case("ChatCompletionRequest"), "chat_completion_request");
        assert!(snake_case(&"A".repeat(200)).len() <= 96);
    }

    #[test]
    fn atomic_items_may_exceed_layout_target_but_have_a_hard_cap() {
        assert!(TARGET_RUST_LINES < MAX_ATOMIC_RUST_LINES);
    }
}
