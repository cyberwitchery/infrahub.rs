//! schema index and identifier claiming

use crate::conventions::{collect_models, relationships, Relationships};
use crate::selection::required_fields;
use graphql_parser::schema::{Definition, Document, Field, InputValue, TypeDefinition};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) struct SchemaContext<'a> {
    pub(crate) types: BTreeMap<String, TypeDefinition<'a, String>>,
    pub(crate) query_type: String,
    pub(crate) mutation_type: Option<String>,
    pub(crate) enums: BTreeSet<String>,
    pub(crate) inputs: BTreeSet<String>,
    pub(crate) objects: BTreeSet<String>,
    pub(crate) interfaces: BTreeSet<String>,
    pub(crate) unions: BTreeSet<String>,
    pub(crate) scalars: BTreeSet<String>,
    type_idents: BTreeMap<String, String>,
    helper_idents: BTreeMap<String, String>,
    response_idents: BTreeMap<(bool, String), String>,
    pub(crate) relationship_edges: BTreeSet<String>,
    pub(crate) relationship_wrappers: BTreeSet<String>,
    pub(crate) required: BTreeMap<String, BTreeSet<String>>,
}

/// the struct every relationship peer is read into.
pub(crate) const PEER_REF: &str = "PeerRef";
/// names the generated code uses unqualified beside the schema's types.
const RESERVED_TYPE_NAMES: &[&str] = &[
    "Box",
    "BoxExtract",
    "BoxFetch",
    "BoxFutureResult",
    "Client",
    "Deserialize",
    "DynPaginator",
    "EdgePage",
    "Error",
    "GeneratedClient",
    "GeneratedClientImpl",
    "GraphQlResponse",
    "Into",
    "None",
    "Ok",
    "Option",
    "Result",
    "Serialize",
    "Some",
    "String",
    "Value",
    "Vec",
];

impl<'a> SchemaContext<'a> {
    pub(crate) fn new(doc: &'a Document<'a, String>) -> Self {
        let mut types = BTreeMap::new();
        let mut enums = BTreeSet::new();
        let mut inputs = BTreeSet::new();
        let mut objects = BTreeSet::new();
        let mut interfaces = BTreeSet::new();
        let mut unions = BTreeSet::new();
        let mut scalars = BTreeSet::new();
        let mut query_type = "Query".to_string();
        let mut mutation_type = None;
        let mut has_schema_definition = false;

        for def in &doc.definitions {
            if let Definition::TypeDefinition(ty) = def {
                let name = match ty {
                    TypeDefinition::Enum(enum_ty) => {
                        enums.insert(enum_ty.name.clone());
                        enum_ty.name.clone()
                    }
                    TypeDefinition::InputObject(input_ty) => {
                        inputs.insert(input_ty.name.clone());
                        input_ty.name.clone()
                    }
                    TypeDefinition::Object(obj) => {
                        objects.insert(obj.name.clone());
                        obj.name.clone()
                    }
                    TypeDefinition::Interface(iface) => {
                        interfaces.insert(iface.name.clone());
                        iface.name.clone()
                    }
                    TypeDefinition::Union(union_ty) => {
                        unions.insert(union_ty.name.clone());
                        union_ty.name.clone()
                    }
                    TypeDefinition::Scalar(scalar_ty) => {
                        scalars.insert(scalar_ty.name.clone());
                        scalar_ty.name.clone()
                    }
                };
                types.insert(name, ty.clone());
            } else if let Definition::SchemaDefinition(schema) = def {
                has_schema_definition = true;
                if let Some(query) = &schema.query {
                    query_type = query.to_string();
                }
                mutation_type = schema.mutation.as_ref().map(|m| m.to_string());
            }
        }

        if !has_schema_definition && objects.contains("Mutation") {
            mutation_type = Some("Mutation".to_string());
        }

        let mut taken = RESERVED_TYPE_NAMES
            .iter()
            .map(|name| name.to_string())
            .collect();
        let type_idents = types
            .keys()
            .map(|name| (name.clone(), claim(&mut taken, name)))
            .collect();
        let mut ctx = Self {
            types,
            query_type,
            mutation_type,
            enums,
            inputs,
            objects,
            interfaces,
            unions,
            scalars,
            type_idents,
            helper_idents: BTreeMap::new(),
            response_idents: BTreeMap::new(),
            relationship_edges: BTreeSet::new(),
            relationship_wrappers: BTreeSet::new(),
            required: BTreeMap::new(),
        };
        let Relationships { edges, wrappers } = relationships(&ctx);
        ctx.relationship_edges = edges;
        ctx.relationship_wrappers = wrappers;
        ctx.required = required_fields(&ctx);
        ctx.claim_helper_idents(taken);
        ctx
    }

    /// names the generated structs from what the schema's types left in `taken`.
    fn claim_helper_idents(&mut self, mut taken: BTreeSet<String>) {
        let models = collect_models(self);
        let namespaces: BTreeSet<String> = models
            .values()
            .map(|model| to_snake(&model.namespace))
            .collect();
        let mut helpers: Vec<String> = namespaces
            .iter()
            .map(|ns| format!("{}Api", to_rust_ident(ns)))
            .collect();
        for model in models.keys() {
            helpers.push(format!("{model}Client"));
            helpers.push(format!("{model}Filters"));
        }
        if !self.relationship_edges.is_empty() {
            helpers.push(PEER_REF.to_string());
        }
        let mut root_fields = Vec::new();
        for (is_mutation, root) in [
            (false, Some(&self.query_type)),
            (true, self.mutation_type.as_ref()),
        ] {
            if let Some(TypeDefinition::Object(root)) = root.and_then(|root| self.types.get(root)) {
                root_fields.extend(
                    root.fields
                        .iter()
                        .map(|field| (is_mutation, field.name.clone())),
                );
            }
        }
        for name in helpers {
            let ident = claim(&mut taken, &name);
            self.helper_idents.insert(name, ident);
        }
        root_fields.sort_by_key(|(_, field)| to_snake(field) != *field);
        for (is_mutation, field) in root_fields {
            let ident = claim(&mut taken, &format!("{}Response", to_rust_ident(&field)));
            self.response_idents.insert((is_mutation, field), ident);
        }
    }

    /// the rust identifier of a schema type.
    pub(crate) fn type_ident(&self, name: &str) -> &str {
        &self.type_idents[name]
    }

    /// the identifier of a generated struct, by the name it takes when that is free.
    pub(crate) fn helper_ident(&self, name: &str) -> &str {
        &self.helper_idents[name]
    }

    /// the identifier of a root field's response struct.
    pub(crate) fn response_ident(&self, is_mutation: bool, field: &str) -> &str {
        &self.response_idents[&(is_mutation, field.to_string())]
    }
}

pub(crate) fn to_snake(name: &str) -> String {
    split_identifier_words(name)
        .into_iter()
        .map(|w| w.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

pub(crate) fn split_identifier_words(name: &str) -> Vec<String> {
    let chars: Vec<char> = name.chars().collect();
    if chars.is_empty() {
        return Vec::new();
    }

    let mut words = Vec::new();
    let mut start = 0usize;

    for i in 1..chars.len() {
        let prev = chars[i - 1];
        let curr = chars[i];

        if curr == '_' || curr == '-' {
            if start < i {
                words.push(chars[start..i].iter().collect::<String>());
            }
            start = i + 1;
            continue;
        }

        let next = chars.get(i + 1).copied();
        let lower_to_upper = prev.is_ascii_lowercase() && curr.is_ascii_uppercase();
        let acronym_to_word = prev.is_ascii_uppercase()
            && curr.is_ascii_uppercase()
            && next.map(|c| c.is_ascii_lowercase()).unwrap_or(false);
        let digit_to_alpha = prev.is_ascii_digit() && curr.is_ascii_alphabetic();

        if lower_to_upper || acronym_to_word || digit_to_alpha {
            words.push(chars[start..i].iter().collect::<String>());
            start = i;
        }
    }

    if start < chars.len() {
        words.push(chars[start..].iter().collect::<String>());
    }

    words.retain(|w| !w.is_empty());
    words
}

pub(crate) fn fields_of<'c, 'a>(name: &str, ctx: &'c SchemaContext<'a>) -> &'c [Field<'a, String>] {
    match ctx.types.get(name) {
        Some(TypeDefinition::Object(obj)) => &obj.fields,
        Some(TypeDefinition::Interface(iface)) => &iface.fields,
        _ => &[],
    }
}

pub(crate) fn to_rust_ident(name: &str) -> String {
    let words = split_identifier_words(name);
    let all_upper = words
        .iter()
        .all(|w| w.chars().all(|c| !c.is_ascii_lowercase()));
    let out: String = words
        .into_iter()
        .map(|w| {
            if !all_upper && w.chars().all(|c| !c.is_ascii_lowercase()) {
                return w;
            }
            let mut chars = w.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    let lower: String = chars.map(|c| c.to_ascii_lowercase()).collect();
                    format!("{}{}", upper, lower)
                }
            }
        })
        .collect();
    match out.as_str() {
        "Self" | "Type" | "Box" | "Result" => format!("{}Type", out),
        _ => out,
    }
}

pub(crate) fn to_rust_field(name: &str) -> String {
    escape_keyword(&to_snake(name))
}

fn escape_keyword(snake: &str) -> String {
    match snake {
        "_" | "crate" | "self" | "super" => format!("{}_", snake),
        _ if is_rust_keyword(snake) => format!("r#{}", snake),
        _ => snake.to_string(),
    }
}

/// `to_rust_field` for every name in one scope, in order, without duplicates or `reserved` names.
/// a name already in snake case keeps its identifier; the others append `_` until free.
pub(crate) fn scope_names<'n>(
    names: impl IntoIterator<Item = &'n str>,
    reserved: &[&str],
) -> Vec<String> {
    scope_idents(names, reserved, to_snake, escape_keyword)
}

/// `scope_names` with another case conversion and keyword escape.
pub(crate) fn scope_idents<'n>(
    names: impl IntoIterator<Item = &'n str>,
    reserved: &[&str],
    convert: fn(&str) -> String,
    escape: fn(&str) -> String,
) -> Vec<String> {
    let names: Vec<&str> = names.into_iter().collect();
    let mut taken: BTreeSet<String> = reserved.iter().map(|name| name.to_string()).collect();
    let mut out = vec![String::new(); names.len()];
    let (converted, rest): (Vec<usize>, Vec<usize>) =
        (0..names.len()).partition(|&i| convert(names[i]) == names[i]);
    for i in converted.into_iter().chain(rest) {
        let mut base = convert(names[i]);
        out[i] = loop {
            let ident = escape(&base);
            if taken.insert(ident.clone()) {
                break ident;
            }
            base.push('_');
        };
    }
    out
}

/// the first of `name`, `name_`, `name__`, … that is neither a keyword nor in `taken`, added to `taken`.
fn claim(taken: &mut BTreeSet<String>, name: &str) -> String {
    let mut ident = name.to_string();
    while is_rust_keyword(&ident) || !taken.insert(ident.clone()) {
        ident.push('_');
    }
    ident
}

/// `scope_names` for a generated method's arguments, beside its own `request_branch` and `vars`.
pub(crate) fn method_arg_names(args: &[InputValue<String>]) -> Vec<String> {
    scope_names(
        args.iter().map(|arg| arg.name.as_str()),
        &["request_branch", "vars"],
    )
}

fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "gen"
            | "macro"
            | "override"
            | "priv"
            | "try"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

#[cfg(test)]
mod scope_name_tests {
    use super::*;
    use crate::emit::{render_api_mod, render_api_modules, render_client, render_inputs};
    use graphql_parser::schema::parse_schema;

    const COLLIDING: &str = include_str!("../../../tests/fixtures/colliding_names.graphql");

    #[test]
    fn test_scope_names_prefer_names_already_in_snake_case() {
        assert_eq!(
            scope_names(["nodeId", "node_id", "Self", "self", "Type", "type"], &[]),
            ["node_id_", "node_id", "self__", "self_", "type_", "r#type"]
        );
        assert_eq!(scope_names(["_", "__"], &[]), ["__", "___"]);
    }

    #[test]
    fn test_scope_names_step_around_reserved_names() {
        assert_eq!(
            scope_names(
                ["requestBranch", "request_branch", "vars", "query"],
                &["request_branch", "vars"]
            ),
            ["request_branch__", "request_branch_", "vars_", "query"]
        );
    }

    #[test]
    fn test_namespace_modules_are_keyword_safe() {
        let doc = parse_schema::<String>(COLLIDING).unwrap();
        let ctx = SchemaContext::new(&doc);
        let api_mod = render_api_mod(&ctx);
        for line in [
            "pub mod crate_;",
            "pub mod self_;",
            "pub mod r#type;",
            "    pub fn self_(&self) -> self_::SelfTypeApi<'a> {",
            "    pub fn r#type(&self) -> r#type::TypeTypeApi<'a> {",
            "        r#type::TypeTypeApi::new(self.client)",
        ] {
            assert!(api_mod.contains(line), "missing `{line}` in:\n{api_mod}");
        }
        let stems: Vec<_> = render_api_modules(&ctx).into_keys().collect();
        assert_eq!(
            stems,
            ["crate_", "infra", "ipam", "policy", "self_", "type"]
        );
    }

    #[test]
    fn test_model_accessors_are_keyword_safe_and_distinct() {
        let doc = parse_schema::<String>(COLLIDING).unwrap();
        let ctx = SchemaContext::new(&doc);
        let modules = render_api_modules(&ctx);
        let accessors = |stem: &str| {
            modules[stem]
                .lines()
                .filter(|line| line.starts_with("    pub fn ") && line.contains("(&self) -> "))
                .map(str::trim)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            accessors("infra"),
            [
                "pub fn infra(&self) -> InfraClient<'a> {",
                "pub fn infra_(&self) -> InfraInfraClient<'a> {",
                "pub fn new_(&self) -> InfraNewClient<'a> {",
                "pub fn r#type(&self) -> InfraTypeClient<'a> {",
            ]
        );
        assert_eq!(
            accessors("ipam"),
            [
                "pub fn prefix(&self) -> IPAMPrefixClient<'a> {",
                "pub fn prefix_(&self) -> IpamPrefixClient<'a> {",
            ]
        );
        assert_eq!(
            accessors("policy"),
            ["pub fn r#match(&self) -> PolicyMatchClient<'a> {"]
        );
    }

    #[test]
    fn test_method_arguments_step_around_request_branch_and_vars() {
        let doc = parse_schema::<String>(COLLIDING).unwrap();
        let ctx = SchemaContext::new(&doc);
        let client = render_client(&ctx);
        for line in [
            "    pub async fn pong(&self, vars_: String, request_branch_: String , request_branch: Option<&str>)",
            "        vars.insert(\"vars\".to_string(), serde_json::to_value(vars_)?);",
            "        vars.insert(\"requestBranch\".to_string(), serde_json::to_value(request_branch_)?);",
        ] {
            assert!(client.contains(line), "missing `{line}` in:\n{client}");
        }
        let infra = &render_api_modules(&ctx)["infra"];
        assert!(infra.contains(
            "    pub async fn create(&self, data: WidgetInput, request_branch_: Option<String>, vars_: Option<String>, request_branch: Option<&str>)"
        ));
        assert!(infra.contains("        if let Some(value) = vars_ {\n"));
    }

    #[test]
    fn test_root_methods_are_distinct() {
        let doc = parse_schema::<String>(COLLIDING).unwrap();
        let ctx = SchemaContext::new(&doc);
        let client = render_client(&ctx);
        assert!(client.contains("    pub async fn ipam_prefix(&self, ids: Option<Vec<String>> , request_branch: Option<&str>) -> Result<GraphQlResponse<IPAMPrefixResponse>>"));
        assert!(client.contains("    pub async fn ipam_prefix_(&self, ids: Option<Vec<String>> , request_branch: Option<&str>) -> Result<GraphQlResponse<IpamPrefixResponse>>"));
    }

    #[test]
    fn test_struct_fields_are_distinct_and_keep_their_wire_names() {
        let doc = parse_schema::<String>(COLLIDING).unwrap();
        let ctx = SchemaContext::new(&doc);
        let infra = &render_api_modules(&ctx)["infra"];
        for line in [
            "        if let Some(value) = &self.after_ {\n            vars.insert(\"After\"",
            "        if let Some(value) = &self.after {\n            vars.insert(\"after\"",
            "        if let Some(value) = &self.type_ {\n            vars.insert(\"Type\"",
            "            page_filters.after = cursor.clone();\n",
        ] {
            assert!(infra.contains(line), "missing `{line}` in infra");
        }
        let inputs = render_inputs(&ctx);
        for line in [
            "    #[serde(rename = \"nodeId\")]\n    pub node_id_: Option<String>,",
            "    pub node_id: Option<String>,",
            "    #[serde(rename = \"Self\")]\n    pub self__: Option<String>,",
            "    #[serde(rename = \"self\")]\n    pub self_: Option<String>,",
        ] {
            assert!(inputs.contains(line), "missing `{line}` in:\n{inputs}");
        }
    }
}

#[cfg(test)]
mod type_name_tests {
    use super::*;
    use crate::emit::{
        render_api_mod, render_api_modules, render_client, render_inputs, render_responses,
        render_types,
    };
    use graphql_parser::schema::parse_schema;

    const COLLIDING_TYPES: &str =
        include_str!("../../../tests/fixtures/colliding_type_names.graphql");

    fn assert_contains(source: &str, lines: &[&str]) {
        for line in lines {
            assert!(source.contains(line), "missing `{line}` in:\n{source}");
        }
    }

    #[test]
    fn test_claim_skips_keywords_and_taken_names() {
        let mut taken = BTreeSet::from(["Error".to_string()]);
        assert_eq!(claim(&mut taken, "Error"), "Error_");
        assert_eq!(claim(&mut taken, "Error"), "Error__");
        assert_eq!(claim(&mut taken, "Self"), "Self_");
        assert_eq!(claim(&mut taken, "type"), "type_");
        assert_eq!(claim(&mut taken, "Device"), "Device");
    }

    #[test]
    fn test_unqualified_names_beside_schema_types_are_reserved() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        let declared: BTreeSet<&str> = ctx
            .type_idents
            .values()
            .chain(ctx.helper_idents.values())
            .chain(ctx.response_idents.values())
            .map(String::as_str)
            .collect();
        let mut sources = vec![
            render_inputs(&ctx),
            render_responses(&ctx),
            render_client(&ctx),
        ];
        sources.extend(render_api_modules(&ctx).into_values());
        for source in sources {
            let mut code = String::new();
            let mut rest = source.as_str();
            while let Some(quote) = rest.find('"') {
                code.push_str(&rest[..quote]);
                let close = if code.ends_with("r#") { "\"#" } else { "\"" };
                rest = &rest[quote + 1..];
                rest = &rest[rest.find(close).unwrap() + close.len()..];
            }
            code.push_str(rest);
            for (i, part) in code.split("::").enumerate() {
                let part = if i == 0 {
                    part
                } else {
                    part.trim_start_matches(|c: char| c.is_ascii_alphanumeric() || c == '_')
                };
                for name in part
                    .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                    .filter(|name| name.starts_with(|c: char| c.is_ascii_uppercase()))
                {
                    assert!(
                        declared.contains(name)
                            || RESERVED_TYPE_NAMES.contains(&name)
                            || ["Clone", "Debug", "Default", "Self"].contains(&name),
                        "`{name}` is neither declared nor reserved"
                    );
                }
            }
        }
    }

    #[test]
    fn test_helper_structs_yield_to_schema_types() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_contains(
            &render_api_mod(&ctx),
            &[
                "    pub fn infra(&self) -> infra::InfraApi_<'a> {",
                "        infra::InfraApi_::new(self.client)",
            ],
        );
        assert_contains(
            &render_api_modules(&ctx)["infra"],
            &[
                "pub struct InfraApi_<'a> {",
                "impl<'a> InfraApi_<'a> {",
                "    pub fn device(&self) -> InfraDeviceClient_<'a> {\n        InfraDeviceClient_::new(self.client)",
                "pub struct InfraDeviceFilters_ {",
                "impl InfraDeviceFilters_ {",
                "pub struct InfraDeviceClient_<'a> {",
                "impl<'a> InfraDeviceClient_<'a> {",
                "    pub async fn list(&self, filters: Option<InfraDeviceFilters_>, request_branch: Option<&str>) -> Result<Vec<InfraDevice>> {",
                "    pub fn paginate(&self, filters: Option<InfraDeviceFilters_>, ",
                "        let mut filters = InfraDeviceFilters_::default();",
                "request_branch: Option<&str>) -> Result<Vec<InfraApi>> {",
                "request_branch: Option<&str>) -> Result<Vec<InfraDeviceClient>> {",
                "request_branch: Option<&str>) -> Result<Vec<InfraDeviceFilters>> {",
            ],
        );
        assert_contains(
            &render_types(&ctx),
            &[
                "pub struct PeerRef {",
                "pub struct PeerRef_ {",
                "    pub node: Option<Box<PeerRef_>>,",
            ],
        );
    }

    #[test]
    fn test_schema_types_step_around_reserved_names_and_keywords() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_contains(
            &render_types(&ctx),
            &[
                "pub struct Error_ {",
                "pub struct EdgedError {\n    pub node: Option<Box<Error_>>,",
                "pub enum Self_ {",
                "    pub status: Option<Self_>,",
                "pub struct Box_(pub serde_json::Value);",
                "    pub peers: Option<Vec<Box_>>,",
                "pub struct NestedEdgedResult {\n    pub node: Option<Box<Result_>>,",
            ],
        );
        assert_contains(&render_inputs(&ctx), &["pub struct Value_ {"]);
        let modules = render_api_modules(&ctx);
        assert_contains(
            &modules["error"],
            &["request_branch: Option<&str>) -> Result<Vec<Error_>> {"],
        );
        assert_contains(
            &modules["infra"],
            &["    pub async fn create(&self, data: Value_, request_branch: Option<&str>)"],
        );
    }

    #[test]
    fn test_root_fields_get_distinct_response_structs() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_contains(
            &render_responses(&ctx),
            &[
                "pub struct PingResponse_ {\n    pub ping: Option<Box<PingResponse>>,\n}",
                "pub struct PingResponse__ {\n    pub ping: Option<String>,\n}",
                "pub struct FooBarResponse {\n    pub foo_bar: Option<String>,\n}",
                "pub struct FooBarResponse_ {\n    #[serde(rename = \"fooBar\")]\n    pub foo_bar: Option<String>,\n}",
                "pub struct InfraDeviceResponse {\n    pub infra_device: Option<String>,\n}",
                "pub struct InfraDeviceResponse_ {\n    #[serde(rename = \"InfraDevice\")]\n",
                "pub struct InfraDeviceCreateResponse {\n    #[serde(rename = \"InfraDeviceCreate\")]\n    pub infra_device_create: Option<String>,\n}",
                "pub struct InfraDeviceCreateResponse_ {\n    #[serde(rename = \"InfraDeviceCreate\")]\n    pub infra_device_create: Option<Box<InfraDeviceCreate>>,\n}",
            ],
        );
        let infra = &render_api_modules(&ctx)["infra"];
        assert_contains(
            infra,
            &[
                "        let response = self.client.execute::<InfraDeviceResponse_>(query, Some(vars), request_branch).await?;",
                "DynPaginator<'a, InfraDevice, String, (InfraDeviceResponse_, i64)>",
                "        let response = self.client.execute::<InfraDeviceCreateResponse_>(query, Some(vars), request_branch).await?;",
            ],
        );
        assert_contains(
            &render_client(&ctx),
            &[
                "    pub async fn ping(&self, value: Option<Value_> , request_branch: Option<&str>) -> Result<GraphQlResponse<PingResponse_>> {",
                "    pub async fn ping_(&self , request_branch: Option<&str>) -> Result<GraphQlResponse<PingResponse__>> {",
                "    pub async fn foo_bar(&self , request_branch: Option<&str>) -> Result<GraphQlResponse<FooBarResponse>> {",
                "    pub async fn foo_bar_(&self , request_branch: Option<&str>) -> Result<GraphQlResponse<FooBarResponse_>> {",
            ],
        );
    }

    #[test]
    fn test_object_and_interface_fields_are_distinct() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_contains(
            &render_types(&ctx),
            &[
                "pub struct InfraDevice {\n    pub id: String,\n    pub typename: Option<String>,\n    #[serde(rename = \"nodeId\")]\n    pub node_id_: Option<String>,\n    pub node_id: Option<String>,\n",
                "pub struct Result_ {\n    #[serde(rename = \"__typename\")]\n    pub typename: Option<String>,\n    pub id: Option<String>,\n    #[serde(rename = \"typename\")]\n    pub typename_: Option<String>,\n    #[serde(rename = \"nodeId\")]\n    pub node_id_: Option<String>,\n    pub node_id: Option<String>,\n}",
            ],
        );
    }

    #[test]
    fn test_enum_variants_are_distinct() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_contains(
            &render_types(&ctx),
            &["pub enum Self_ {\n    #[serde(rename = \"IN_PROGRESS\")]\n    InProgress_,\n    #[serde(rename = \"InProgress\")]\n    InProgress,\n    #[serde(rename = \"ACTIVE\")]\n    Active,\n    #[serde(rename = \"active\")]\n    Active_,\n}"],
        );
    }

    #[test]
    fn test_namespace_mod_keeps_the_api_root_file() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_contains(
            &render_api_mod(&ctx),
            &[
                "pub mod mod_;",
                "    pub fn mod_(&self) -> mod_::ModApi<'a> {",
            ],
        );
        let stems: Vec<_> = render_api_modules(&ctx).into_keys().collect();
        assert_eq!(stems, ["error", "infra", "mod_"]);
    }
}
