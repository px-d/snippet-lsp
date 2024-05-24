use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, Documentation, MarkupContent, MarkupKind,
};

pub fn load_snippets() -> Vec<CompletionItem> {
    let console_log = CompletionItem {
        label: "Console Log".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Log a message to console".to_string()),
        documentation: Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "```js\nconsole.log(\"Hello, World!\");\n```".to_string(),
        })),
        filter_text: Some("log".to_string()),
        insert_text: Some("console.log('$1');".to_string()),
        ..Default::default()
    };

    let async_arrow_function = CompletionItem {
        label: "Async Arrow Function".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Create an async arrow function".to_string()),
        documentation: Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "```js\nconst myFunction = async () => {}\n```".to_string(),
        })),
        filter_text: Some("aaf".to_string()),
        insert_text: Some("const $1 = async () => {\n\t$2\n}".to_string()),
        ..Default::default()
    };

    let async_function = CompletionItem {
        label: "Async Function".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Create an async function".to_string()),
        documentation: Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "```js\nasync function myFunction() {}\n```".to_string(),
        })),
        filter_text: Some("af".to_string()),
        insert_text: Some("async function $1() {}".to_string()),
        ..Default::default()
    };

    let normal_function = CompletionItem {
        label: "Normal Function".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Create a normal function".to_string()),
        documentation: Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "```js\nfunction myFunction() {}\n```".to_string(),
        })),
        filter_text: Some("fn".to_string()),
        insert_text: Some("function $1() {}".to_string()),
        ..Default::default()
    };

    let async_normal_function = CompletionItem {
        label: "Async Normal Function".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Create an async normal function".to_string()),
        documentation: Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "```js\nasync function myFunction() {}\n```".to_string(),
        })),
        filter_text: Some("afn".to_string()),
        insert_text: Some("async function $1() {}".to_string()),
        ..Default::default()
    };

    vec![
        console_log,
        async_function,
        async_arrow_function,
        normal_function,
        async_normal_function,
    ]
}
