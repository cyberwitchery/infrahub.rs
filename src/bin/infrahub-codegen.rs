//! generate a typed infrahub client from a schema
//!
//! this binary produces a standalone, schema-specific crate with:
//! - typed `types`, `inputs`, and `responses`
//! - a full surface `generated()` client
//! - an ergonomic `api()` layer grouped by schema namespaces
//!
//! command help reference (kept in sync with `infrahub-codegen --help`):
#[doc = concat!("```text\n", include_str!("infrahub-codegen-help.txt"), "\n```")]
pub const CLI_HELP: &str = include_str!("infrahub-codegen-help.txt");

use graphql_parser::schema::{
    parse_schema, Definition, Document, Field, InputObjectType, InputValue, Type, TypeDefinition,
    UnionType,
};
use reqwest::blocking::Client as BlockingClient;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Args {
    url: Option<String>,
    token: Option<String>,
    branch: Option<String>,
    schema_path: Option<PathBuf>,
    out_dir: PathBuf,
    crate_name: Option<String>,
    infrahub_path: Option<String>,
}

enum ParseArgsError {
    Help,
    Message(String),
}

fn main() {
    let args = match parse_args(std::env::args().collect()) {
        Ok(args) => args,
        Err(ParseArgsError::Help) => {
            print!("{CLI_HELP}");
            return;
        }
        Err(ParseArgsError::Message(err)) => {
            eprintln!("{err}\n\n{CLI_HELP}");
            std::process::exit(1);
        }
    };

    let schema = match load_schema(&args) {
        Ok(schema) => schema,
        Err(err) => {
            eprintln!("failed to load schema: {err}");
            std::process::exit(1);
        }
    };

    let document = match parse_schema::<String>(&schema) {
        Ok(doc) => doc,
        Err(err) => {
            eprintln!("failed to parse schema: {err}");
            std::process::exit(1);
        }
    };

    let ctx = SchemaContext::new(&document);

    if let Err(err) = generate_client(&args, &ctx) {
        eprintln!("codegen failed: {err}");
        std::process::exit(1);
    }
}

fn parse_args(args: Vec<String>) -> Result<Args, ParseArgsError> {
    let mut url = None;
    let mut token = None;
    let mut branch = None;
    let mut schema_path = None;
    let mut out_dir = None;
    let mut crate_name = None;
    let mut infrahub_path = None;

    let mut iter = args.into_iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--url" => url = iter.next(),
            "--token" => token = iter.next(),
            "--branch" => branch = iter.next(),
            "--schema" => schema_path = iter.next().map(PathBuf::from),
            "--out" => out_dir = iter.next().map(PathBuf::from),
            "--crate-name" => crate_name = iter.next(),
            "--infrahub-path" => infrahub_path = iter.next(),
            "--help" | "-h" => return Err(ParseArgsError::Help),
            _ => return Err(ParseArgsError::Message(format!("unknown argument: {arg}"))),
        }
    }

    let out_dir =
        out_dir.ok_or_else(|| ParseArgsError::Message("--out is required".to_string()))?;

    if url.is_none() && schema_path.is_none() {
        return Err(ParseArgsError::Message(
            "--url or --schema is required".to_string(),
        ));
    }

    Ok(Args {
        url,
        token,
        branch,
        schema_path,
        out_dir,
        crate_name,
        infrahub_path,
    })
}

fn load_schema(args: &Args) -> Result<String, String> {
    if let Some(schema_path) = &args.schema_path {
        return fs::read_to_string(schema_path)
            .map_err(|err| format!("failed to read {}: {err}", schema_path.display()));
    }

    let url = args
        .url
        .as_ref()
        .ok_or_else(|| "--url is required when --schema not provided".to_string())?;

    let mut schema_url = url.trim_end_matches('/').to_string();
    schema_url.push_str("/schema.graphql");
    if let Some(branch) = &args.branch {
        schema_url.push_str("?branch=");
        schema_url.push_str(branch);
    }

    let mut headers = HeaderMap::new();
    if let Some(token) = &args.token {
        headers.insert(
            "X-INFRAHUB-KEY",
            HeaderValue::from_str(token).map_err(|err| err.to_string())?,
        );
    }

    let client = BlockingClient::new();
    let response = client
        .get(schema_url)
        .headers(headers)
        .send()
        .map_err(|err| err.to_string())?;

    response
        .text()
        .map_err(|err| format!("failed to read schema response: {err}"))
}

struct SchemaContext<'a> {
    types: BTreeMap<String, TypeDefinition<'a, String>>,
    query_type: String,
    mutation_type: Option<String>,
    enums: BTreeSet<String>,
    inputs: BTreeSet<String>,
    objects: BTreeSet<String>,
    unions: BTreeSet<String>,
    scalars: BTreeSet<String>,
}

#[derive(Clone, Debug)]
struct ModelInfo<'a> {
    name: String,
    namespace: String,
    query_field: Option<Field<'a, String>>,
    query_return: Option<String>,
    node_type: String,
    node_boxed: bool,
    create: Option<Field<'a, String>>,
    update: Option<Field<'a, String>>,
    upsert: Option<Field<'a, String>>,
    delete: Option<Field<'a, String>>,
}

impl<'a> SchemaContext<'a> {
    fn new(doc: &'a Document<'a, String>) -> Self {
        let mut types = BTreeMap::new();
        let mut enums = BTreeSet::new();
        let mut inputs = BTreeSet::new();
        let mut objects = BTreeSet::new();
        let mut unions = BTreeSet::new();
        let mut scalars = BTreeSet::new();
        let mut query_type = "Query".to_string();
        let mut mutation_type = None;

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
                    TypeDefinition::Union(union_ty) => {
                        unions.insert(union_ty.name.clone());
                        union_ty.name.clone()
                    }
                    TypeDefinition::Scalar(scalar_ty) => {
                        scalars.insert(scalar_ty.name.clone());
                        scalar_ty.name.clone()
                    }
                    _ => continue,
                };
                types.insert(name, ty.clone());
            } else if let Definition::SchemaDefinition(schema) = def {
                if let Some(query) = &schema.query {
                    query_type = query.to_string();
                }
                mutation_type = schema.mutation.as_ref().map(|m| m.to_string());
            }
        }

        Self {
            types,
            query_type,
            mutation_type,
            enums,
            inputs,
            objects,
            unions,
            scalars,
        }
    }
}

fn generate_client(args: &Args, ctx: &SchemaContext) -> Result<(), String> {
    let out_dir = &args.out_dir;
    let src_dir = out_dir.join("src");
    let api_dir = src_dir.join("api");
    fs::create_dir_all(&src_dir).map_err(|err| err.to_string())?;
    fs::create_dir_all(&api_dir).map_err(|err| err.to_string())?;

    if let Some(crate_name) = &args.crate_name {
        let mut cargo = String::new();
        cargo.push_str("[package]\n");
        cargo.push_str(&format!("name = \"{}\"\n", crate_name));
        cargo.push_str("version = \"0.0.1\"\n");
        cargo.push_str("edition = \"2021\"\n\n");
        cargo.push_str("[dependencies]\n");
        if let Some(path) = &args.infrahub_path {
            cargo.push_str(&format!("infrahub = {{ path = \"{}\" }}\n", path));
        } else {
            cargo.push_str("infrahub = \"0.0.1\"\n");
        }
        cargo.push_str("serde = { version = \"1\", features = [\"derive\"] }\n");
        cargo.push_str("serde_json = \"1\"\n");
        fs::write(out_dir.join("Cargo.toml"), cargo).map_err(|err| err.to_string())?;
    }

    let types_rs = render_types(ctx);
    fs::write(src_dir.join("types.rs"), types_rs).map_err(|err| err.to_string())?;

    let inputs_rs = render_inputs(ctx);
    fs::write(src_dir.join("inputs.rs"), inputs_rs).map_err(|err| err.to_string())?;

    let responses_rs = render_responses(ctx);
    fs::write(src_dir.join("responses.rs"), responses_rs).map_err(|err| err.to_string())?;

    let client_rs = render_client(ctx);
    fs::write(src_dir.join("client.rs"), client_rs).map_err(|err| err.to_string())?;

    let api_mod = render_api_mod(ctx);
    fs::write(api_dir.join("mod.rs"), api_mod).map_err(|err| err.to_string())?;

    let api_modules = render_api_modules(ctx);
    for (name, content) in api_modules {
        fs::write(api_dir.join(format!("{name}.rs")), content).map_err(|err| err.to_string())?;
    }

    let lib_rs = render_lib();
    fs::write(src_dir.join("lib.rs"), lib_rs).map_err(|err| err.to_string())?;

    Ok(())
}

fn render_lib() -> String {
    let mut out = String::new();
    out.push_str("//! generated infrahub client\n\n");
    out.push_str("pub mod api;\n");
    out.push_str("pub mod client;\n");
    out.push_str("pub mod inputs;\n");
    out.push_str("pub mod responses;\n");
    out.push_str("pub mod types;\n\n");
    out.push_str("pub use client::GeneratedClient;\n");
    out.push_str("pub use api::{Api, ApiClient};\n");
    out
}

fn render_types(ctx: &SchemaContext) -> String {
    let mut out = String::new();
    out.push_str("//! generated types\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n\n");

    for enum_name in &ctx.enums {
        if let Some(TypeDefinition::Enum(enum_ty)) = ctx.types.get(enum_name) {
            out.push_str("#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]\n");
            out.push_str(&format!("pub enum {} {{\n", enum_name));
            for value in &enum_ty.values {
                let variant = to_rust_ident(&value.name);
                out.push_str(&format!("    #[serde(rename = \"{}\")]\n", value.name));
                out.push_str(&format!("    {},\n", variant));
            }
            out.push_str("}\n\n");
        }
    }

    for obj_name in &ctx.objects {
        if obj_name == &ctx.query_type {
            continue;
        }
        if let Some(TypeDefinition::Object(obj)) = ctx.types.get(obj_name) {
            out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
            out.push_str(&format!("pub struct {} {{\n", obj_name));
            for field in &obj.fields {
                let rust_name = to_rust_field(field.name.as_str());
                let ty = rust_type(&field.field_type, ctx, false);
                if rust_name != field.name {
                    out.push_str(&format!("    #[serde(rename = \"{}\")]\n", field.name));
                }
                out.push_str(&format!("    pub {}: {},\n", rust_name, ty));
            }
            out.push_str("}\n\n");
        }
    }

    for union_name in &ctx.unions {
        if let Some(TypeDefinition::Union(UnionType { name, .. })) = ctx.types.get(union_name) {
            out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
            out.push_str(&format!("pub struct {}(pub serde_json::Value);\n\n", name));
        }
    }

    out
}

fn render_inputs(ctx: &SchemaContext) -> String {
    let mut out = String::new();
    out.push_str("//! generated input types\n\n");
    out.push_str("#![allow(non_snake_case)]\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n\n");
    out.push_str("use crate::types::*;\n\n");

    for input_name in &ctx.inputs {
        if let Some(TypeDefinition::InputObject(InputObjectType { name, fields, .. })) =
            ctx.types.get(input_name)
        {
            out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
            out.push_str(&format!("pub struct {} {{\n", name));
            for field in fields {
                let rust_name = to_rust_field(field.name.as_str());
                let ty = rust_type(&field.value_type, ctx, true);
                if rust_name != field.name {
                    out.push_str(&format!("    #[serde(rename = \"{}\")]\n", field.name));
                }
                out.push_str(&format!("    pub {}: {},\n", rust_name, ty));
            }
            out.push_str("}\n\n");
        }
    }

    out
}

fn render_responses(ctx: &SchemaContext) -> String {
    let mut out = String::new();
    out.push_str("//! generated response wrappers\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n\n");
    out.push_str("use crate::types::*;\n\n");

    let query = ctx.types.get(&ctx.query_type).and_then(|ty| match ty {
        TypeDefinition::Object(obj) => Some(obj),
        _ => None,
    });

    if let Some(query) = query {
        for field in &query.fields {
            let resp_name = format!("{}Response", to_rust_ident(field.name.as_str()));
            out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
            out.push_str(&format!("pub struct {} {{\n", resp_name));
            let rust_name = to_rust_field(field.name.as_str());
            let ty = rust_type(&field.field_type, ctx, false);
            if rust_name != field.name {
                out.push_str(&format!("    #[serde(rename = \"{}\")]\n", field.name));
            }
            out.push_str(&format!("    pub {}: {},\n", rust_name, ty));
            out.push_str("}\n\n");
        }
    }

    if let Some(mutation_name) = &ctx.mutation_type {
        if let Some(TypeDefinition::Object(mutation)) = ctx.types.get(mutation_name) {
            for field in &mutation.fields {
                let resp_name = format!("{}Response", to_rust_ident(field.name.as_str()));
                out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
                out.push_str(&format!("pub struct {} {{\n", resp_name));
                let rust_name = to_rust_field(field.name.as_str());
                let ty = rust_type(&field.field_type, ctx, false);
                if rust_name != field.name {
                    out.push_str(&format!("    #[serde(rename = \"{}\")]\n", field.name));
                }
                out.push_str(&format!("    pub {}: {},\n", rust_name, ty));
                out.push_str("}\n\n");
            }
        }
    }

    out
}

fn render_client(ctx: &SchemaContext) -> String {
    let mut out = String::new();
    out.push_str("//! generated client\n\n");
    out.push_str("#![allow(non_snake_case, clippy::too_many_arguments)]\n\n");
    out.push_str("use infrahub::{Client, GraphQlResponse, Result};\n");
    out.push_str("use serde_json::Value;\n\n");
    out.push_str("use crate::inputs::*;\n");
    out.push_str("use crate::responses::*;\n");
    out.push_str("use crate::types::*;\n\n");

    out.push_str("pub trait GeneratedClient {\n");
    out.push_str("    fn generated(&self) -> GeneratedClientImpl<'_>;\n");
    out.push_str("}\n\n");

    out.push_str("impl GeneratedClient for Client {\n");
    out.push_str("    fn generated(&self) -> GeneratedClientImpl<'_> {\n");
    out.push_str("        GeneratedClientImpl { client: self }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("pub struct GeneratedClientImpl<'a> {\n");
    out.push_str("    client: &'a Client,\n");
    out.push_str("}\n\n");

    out.push_str("impl<'a> GeneratedClientImpl<'a> {\n");

    if let Some(query) = ctx.types.get(&ctx.query_type).and_then(|ty| match ty {
        TypeDefinition::Object(obj) => Some(obj),
        _ => None,
    }) {
        for field in &query.fields {
            out.push_str(&render_field_method(field, ctx, false));
        }
    }

    if let Some(mutation_name) = &ctx.mutation_type {
        if let Some(TypeDefinition::Object(mutation)) = ctx.types.get(mutation_name) {
            for field in &mutation.fields {
                out.push_str(&render_field_method(field, ctx, true));
            }
        }
    }

    out.push_str("}\n");

    out
}

fn render_api_mod<'a>(ctx: &SchemaContext<'a>) -> String {
    let models = collect_models(ctx);
    let mut namespaces: BTreeSet<String> = BTreeSet::new();
    for model in models.values() {
        namespaces.insert(to_snake(&model.namespace));
    }

    let mut out = String::new();
    out.push_str("//! generated ergonomic api\n\n");
    out.push_str("use infrahub::Client;\n\n");
    for ns in &namespaces {
        out.push_str(&format!("pub mod {};\n", ns));
    }
    out.push_str("\n");
    out.push_str("pub struct Api<'a> {\n");
    out.push_str("    client: &'a Client,\n");
    out.push_str("}\n\n");
    out.push_str("pub trait ApiClient {\n");
    out.push_str("    fn api(&self) -> Api<'_>;\n");
    out.push_str("}\n\n");
    out.push_str("impl ApiClient for Client {\n");
    out.push_str("    fn api(&self) -> Api<'_> {\n");
    out.push_str("        Api { client: self }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("impl<'a> Api<'a> {\n");
    for ns in &namespaces {
        let struct_name = format!("{}Api", to_rust_ident(ns));
        out.push_str(&format!(
            "    pub fn {}(&self) -> {}::{}<'a> {{\n",
            ns, ns, struct_name
        ));
        out.push_str(&format!(
            "        {}::{}::new(self.client)\n",
            ns, struct_name
        ));
        out.push_str("    }\n");
    }
    out.push_str("}\n");
    out
}

fn render_api_modules<'a>(ctx: &SchemaContext<'a>) -> BTreeMap<String, String> {
    let models = collect_models(ctx);
    let mut by_ns: BTreeMap<String, Vec<ModelInfo<'a>>> = BTreeMap::new();
    for model in models.values() {
        by_ns
            .entry(to_snake(&model.namespace))
            .or_default()
            .push(model.clone());
    }

    let mut out = BTreeMap::new();
    for (ns, mut models) in by_ns {
        models.sort_by(|a, b| a.name.cmp(&b.name));
        out.insert(ns.clone(), render_api_module(&ns, &models, ctx));
    }
    out
}

fn render_api_module<'a>(
    namespace: &str,
    models: &[ModelInfo<'a>],
    ctx: &SchemaContext<'a>,
) -> String {
    let struct_name = format!("{}Api", to_rust_ident(namespace));
    let mut out = String::new();
    out.push_str("//! generated api module\n\n");
    out.push_str("#![allow(non_snake_case, unused_imports)]\n\n");
    out.push_str("use infrahub::{Client, Error, Result};\n");
    out.push_str("use serde_json::Value;\n\n");
    out.push_str("use crate::inputs::*;\n");
    out.push_str("use crate::responses::*;\n");
    out.push_str("use crate::types::*;\n\n");

    out.push_str(&format!("pub struct {}<'a> {{\n", struct_name));
    out.push_str("    client: &'a Client,\n");
    out.push_str("}\n\n");
    out.push_str(&format!("impl<'a> {}<'a> {{\n", struct_name));
    out.push_str("    pub(crate) fn new(client: &'a Client) -> Self {\n");
    out.push_str("        Self { client }\n");
    out.push_str("    }\n\n");
    for model in models {
        let accessor = model_accessor_name(&model.name, &model.namespace);
        let client_struct = format!("{}Client", model.name);
        out.push_str(&format!(
            "    pub fn {}(&self) -> {}<'a> {{\n",
            accessor, client_struct
        ));
        out.push_str(&format!("        {}::new(self.client)\n", client_struct));
        out.push_str("    }\n");
    }
    out.push_str("}\n\n");

    for model in models {
        out.push_str(&render_model_client(model, ctx));
    }

    out
}

fn render_model_client<'a>(model: &ModelInfo<'a>, ctx: &SchemaContext<'a>) -> String {
    let mut out = String::new();
    let client_struct = format!("{}Client", model.name);
    let filters_struct = format!("{}Filters", model.name);
    let model_field = to_rust_field(model.name.as_str());

    if let Some(query_field) = &model.query_field {
        let args = &query_field.arguments;
        out.push_str(&format!(
            "#[derive(Debug, Clone, Default)]\npub struct {} {{\n",
            filters_struct
        ));
        for arg in args {
            let rust_name = to_rust_field(&arg.name);
            let inner = rust_type_nonnull(&arg.value_type, ctx, true, false);
            out.push_str(&format!("    pub {}: Option<{}>,\n", rust_name, inner));
        }
        out.push_str("}\n\n");
        out.push_str(&format!("impl {} {{\n", filters_struct));
        out.push_str("    fn to_vars(&self) -> Value {\n");
        out.push_str("        let mut vars = serde_json::Map::new();\n");
        for arg in args {
            let rust_name = to_rust_field(&arg.name);
            out.push_str(&format!(
                "        if let Some(value) = &self.{rust_name} {{\n"
            ));
            out.push_str(&format!(
                "            vars.insert(\"{}\".to_string(), serde_json::to_value(value).expect(\"serialize\"));\n",
                arg.name
            ));
            out.push_str("        }\n");
        }
        out.push_str("        Value::Object(vars)\n");
        out.push_str("    }\n");
        out.push_str("}\n\n");
    }

    out.push_str(&format!("pub struct {}<'a> {{\n", client_struct));
    out.push_str("    client: &'a Client,\n");
    out.push_str("}\n\n");
    out.push_str(&format!("impl<'a> {}<'a> {{\n", client_struct));
    out.push_str("    pub(crate) fn new(client: &'a Client) -> Self {\n");
    out.push_str("        Self { client }\n");
    out.push_str("    }\n\n");

    if let Some(query_field) = &model.query_field {
        let query_name = query_field.name.clone();
        let vars_def = render_variable_defs(&query_field.arguments);
        let field_args = render_field_args(&query_field.arguments);
        let return_type = model
            .query_return
            .clone()
            .unwrap_or_else(|| "serde_json::Value".to_string());
        let selection = selection_for_type(&return_type, ctx, &mut BTreeSet::new(), 0);
        let op_header = if vars_def.is_empty() {
            format!("query {}", query_name)
        } else {
            format!("query {}({})", query_name, vars_def)
        };

        out.push_str(&format!(
            "    pub async fn list(&self, filters: Option<{filters_struct}>, request_branch: Option<&str>) -> Result<Vec<{model_type}>> {{\n",
            filters_struct = format!("{}Filters", model.name),
            model_type = model.node_type
        ));
        out.push_str("        let vars = filters.map(|f| f.to_vars()).unwrap_or_else(|| Value::Object(serde_json::Map::new()));\n");
        out.push_str(&format!(
            "        let query = r#\"{op} {{ {name}{args} {sel} }}\"#;\n",
            op = op_header,
            name = query_name,
            args = field_args,
            sel = selection
        ));
        out.push_str(&format!(
            "        let response = self.client.execute::<{}Response>(query, Some(vars), request_branch).await?;\n",
            to_rust_ident(&query_name)
        ));
        out.push_str("        let data = response.data.ok_or_else(|| Error::Config(\"missing data\".to_string()))?;\n");
        out.push_str("        let mut items = Vec::new();\n");
        out.push_str(&format!(
            "        for edge in data.{field}.edges {{\n",
            field = model_field
        ));
        out.push_str("            if let Some(node) = edge.node {\n");
        if model.node_boxed {
            out.push_str("                items.push(*node);\n");
        } else {
            out.push_str("                items.push(node);\n");
        }
        out.push_str("            }\n");
        out.push_str("        }\n");
        out.push_str("        Ok(items)\n");
        out.push_str("    }\n\n");

        if query_field.arguments.iter().any(|arg| arg.name == "ids") {
            out.push_str(&format!(
                "    pub async fn get_by_id(&self, id: impl Into<String>, request_branch: Option<&str>) -> Result<Option<{}>> {{\n",
                model.node_type
            ));
            out.push_str(&format!(
                "        let mut filters = {}Filters::default();\n",
                model.name
            ));
            out.push_str("        filters.ids = Some(vec![id.into()]);\n");
            out.push_str(
                "        let mut items = self.list(Some(filters), request_branch).await?;\n",
            );
            out.push_str("        Ok(items.pop())\n");
            out.push_str("    }\n\n");
        }
    }

    out.push_str(&render_mutation_helpers(model, ctx));
    out.push_str("}\n\n");
    out
}

fn render_mutation_helpers<'a>(model: &ModelInfo<'a>, ctx: &SchemaContext<'a>) -> String {
    let mut out = String::new();
    let mutations = [
        ("create", &model.create),
        ("update", &model.update),
        ("upsert", &model.upsert),
        ("delete", &model.delete),
    ];

    for (name, field_opt) in mutations {
        let Some(field) = field_opt else { continue };
        let field_name = field.name.clone();
        let vars_def = render_variable_defs(&field.arguments);
        let field_args = render_field_args(&field.arguments);
        let return_type = base_type_name(&field.field_type);
        let selection = selection_for_type(&return_type, ctx, &mut BTreeSet::new(), 0);
        let (object_type, object_boxed) = object_type_for_return(&return_type, ctx);
        let response_type = format!("{}Response", to_rust_ident(&field_name));
        let response_field = to_rust_field(&field_name);
        let op_header = if vars_def.is_empty() {
            format!("mutation {}", field_name)
        } else {
            format!("mutation {}({})", field_name, vars_def)
        };

        let mut method_args = Vec::new();
        for arg in &field.arguments {
            let rust_name = to_rust_field(&arg.name);
            let ty = rust_type(&arg.value_type, ctx, true);
            method_args.push(format!("{rust_name}: {ty}"));
        }
        method_args.push("request_branch: Option<&str>".to_string());

        let ret = if name == "delete" {
            "bool".to_string()
        } else {
            object_type.clone()
        };
        out.push_str(&format!(
            "    pub async fn {name}(&self, {args}) -> Result<{ret}> {{\n",
            name = name,
            args = method_args.join(", "),
            ret = ret
        ));
        out.push_str("        let mut vars = serde_json::Map::new();\n");
        out.push_str(&render_vars_builder(&field.arguments));
        out.push_str("        let vars = Value::Object(vars);\n");
        out.push_str(&format!(
            "        let query = r#\"{op} {{ {fname}{args} {sel} }}\"#;\n",
            op = op_header,
            fname = field_name,
            args = field_args,
            sel = selection
        ));
        out.push_str(&format!(
            "        let response = self.client.execute::<{resp}>(query, Some(vars), request_branch).await?;\n",
            resp = response_type
        ));
        out.push_str("        let data = response.data.ok_or_else(|| Error::Config(\"missing data\".to_string()))?;\n");
        out.push_str(&format!(
            "        let payload = data.{field}.ok_or_else(|| Error::Config(\"missing payload\".to_string()))?;\n",
            field = response_field
        ));
        if name == "delete" {
            out.push_str("        Ok(payload.ok.unwrap_or(false))\n");
        } else {
            out.push_str("        let object = payload.object.ok_or_else(|| Error::Config(\"missing object\".to_string()))?;\n");
            if object_boxed {
                out.push_str("        Ok(*object)\n");
            } else {
                out.push_str("        Ok(object)\n");
            }
        }
        out.push_str("    }\n\n");
    }

    out
}

fn collect_models<'a>(ctx: &SchemaContext<'a>) -> BTreeMap<String, ModelInfo<'a>> {
    let mut models: BTreeMap<String, ModelInfo<'a>> = BTreeMap::new();

    if let Some(TypeDefinition::Object(query)) = ctx.types.get(&ctx.query_type) {
        for field in &query.fields {
            let return_type = base_type_name(&field.field_type);
            if let Some(model) = return_type.strip_prefix("Paginated") {
                let namespace = namespace_from_type(model);
                let (node_type, node_boxed) = node_type_for_model(model, ctx);
                models
                    .entry(model.to_string())
                    .and_modify(|info| {
                        info.query_field = Some(field.clone());
                        info.query_return = Some(return_type.clone());
                        info.node_type = node_type.clone();
                        info.node_boxed = node_boxed;
                    })
                    .or_insert(ModelInfo {
                        name: model.to_string(),
                        namespace,
                        query_field: Some(field.clone()),
                        query_return: Some(return_type),
                        node_type,
                        node_boxed,
                        create: None,
                        update: None,
                        upsert: None,
                        delete: None,
                    });
            }
        }
    }

    if let Some(mutation_name) = &ctx.mutation_type {
        if let Some(TypeDefinition::Object(mutation)) = ctx.types.get(mutation_name) {
            for field in &mutation.fields {
                let name = field.name.as_str();
                let (model, slot) = if let Some(model) = name.strip_suffix("Create") {
                    (model.to_string(), "create")
                } else if let Some(model) = name.strip_suffix("Update") {
                    (model.to_string(), "update")
                } else if let Some(model) = name.strip_suffix("Upsert") {
                    (model.to_string(), "upsert")
                } else if let Some(model) = name.strip_suffix("Delete") {
                    (model.to_string(), "delete")
                } else {
                    continue;
                };
                let namespace = namespace_from_type(&model);
                let (node_type, node_boxed) = node_type_for_model(&model, ctx);
                let entry = models.entry(model.clone()).or_insert(ModelInfo {
                    name: model.clone(),
                    namespace,
                    query_field: None,
                    query_return: None,
                    node_type,
                    node_boxed,
                    create: None,
                    update: None,
                    upsert: None,
                    delete: None,
                });
                match slot {
                    "create" => entry.create = Some(field.clone()),
                    "update" => entry.update = Some(field.clone()),
                    "upsert" => entry.upsert = Some(field.clone()),
                    "delete" => entry.delete = Some(field.clone()),
                    _ => {}
                }
            }
        }
    }

    models
}

fn namespace_from_type(name: &str) -> String {
    let mut out = String::new();
    let mut prev_lower = false;
    for ch in name.chars() {
        if ch.is_uppercase() && prev_lower {
            break;
        }
        out.push(ch);
        prev_lower = ch.is_lowercase();
    }
    if out.is_empty() {
        name.to_string()
    } else {
        out
    }
}

fn model_accessor_name(model: &str, namespace: &str) -> String {
    let name = model.strip_prefix(namespace).unwrap_or(model);
    let name = if name.is_empty() { model } else { name };
    to_snake(name)
}

fn to_snake(name: &str) -> String {
    let mut out = String::new();
    for (idx, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            if idx > 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn node_type_for_model<'a>(model: &str, ctx: &SchemaContext<'a>) -> (String, bool) {
    let edge_type = format!("Edged{}", model);
    if let Some(TypeDefinition::Object(obj)) = ctx.types.get(&edge_type) {
        if let Some(node_field) = obj.fields.iter().find(|f| f.name == "node") {
            let rust = rust_type(&node_field.field_type, ctx, false);
            return strip_option_box(&rust);
        }
    }
    ("serde_json::Value".to_string(), false)
}

fn object_type_for_return<'a>(return_type: &str, ctx: &SchemaContext<'a>) -> (String, bool) {
    if let Some(TypeDefinition::Object(obj)) = ctx.types.get(return_type) {
        if let Some(field) = obj.fields.iter().find(|f| f.name == "object") {
            let rust = rust_type(&field.field_type, ctx, false);
            return strip_option_box(&rust);
        }
    }
    ("serde_json::Value".to_string(), false)
}

fn strip_option_box(ty: &str) -> (String, bool) {
    let inner = strip_wrapped(ty, "Option<", ">");
    let boxed = inner.starts_with("Box<") && inner.ends_with('>');
    let inner = if boxed {
        strip_wrapped(inner, "Box<", ">").to_string()
    } else {
        inner.to_string()
    };
    (inner, boxed)
}

fn strip_wrapped<'a>(value: &'a str, prefix: &str, suffix: &str) -> &'a str {
    if value.starts_with(prefix) && value.ends_with(suffix) {
        &value[prefix.len()..value.len() - suffix.len()]
    } else {
        value
    }
}

fn render_field_method(field: &Field<String>, ctx: &SchemaContext, is_mutation: bool) -> String {
    let mut out = String::new();
    let method_name = to_rust_field(field.name.as_str());
    let op_name = if is_mutation { "mutation" } else { "query" };
    let query_name = to_rust_ident(field.name.as_str());
    let response_name = format!("{}Response", query_name);

    let args = render_args(&field.arguments, ctx);
    let vars_builder = render_vars_builder(&field.arguments);
    let field_args = render_field_args(&field.arguments);

    let selection = selection_for_field(field, ctx);
    let var_defs = render_variable_defs(&field.arguments);
    let op_header = if var_defs.is_empty() {
        format!("{} {}", op_name, query_name)
    } else {
        format!("{} {}({})", op_name, query_name, var_defs)
    };
    let query = format!(
        "{} {{ {}{}{} }}",
        op_header, field.name, field_args, selection
    );

    out.push_str(&format!("    pub async fn {}(&self{} , request_branch: Option<&str>) -> Result<GraphQlResponse<{}>> {{\n", method_name, args.signature, response_name));
    if field.arguments.is_empty() {
        out.push_str("        let vars = serde_json::Map::new();\n");
    } else {
        out.push_str("        let mut vars = serde_json::Map::new();\n");
        out.push_str(&vars_builder);
    }
    out.push_str(&format!("        let query = r#\"{}\"#;\n", query));
    out.push_str("        let vars = Value::Object(vars);\n");
    out.push_str("        self.client.execute(query, Some(vars), request_branch).await\n");
    out.push_str("    }\n\n");

    out
}

fn render_args(args: &[InputValue<String>], ctx: &SchemaContext) -> MethodArgs {
    let mut signature_parts = Vec::new();
    for arg in args {
        let rust_name = to_rust_field(&arg.name);
        let ty = rust_type(&arg.value_type, ctx, true);
        signature_parts.push(format!("{}: {}", rust_name, ty));
    }
    let signature = if signature_parts.is_empty() {
        "".to_string()
    } else {
        format!(", {}", signature_parts.join(", "))
    };

    MethodArgs { signature }
}

fn render_vars_builder(args: &[InputValue<String>]) -> String {
    let mut out = String::new();
    for arg in args {
        let rust_name = to_rust_field(&arg.name);
        let var_name = &arg.name;
        if is_optional(&arg.value_type) {
            out.push_str(&format!("        if let Some(value) = {} {{\n", rust_name));
            out.push_str(&format!(
                "            vars.insert(\"{}\".to_string(), serde_json::to_value(value).expect(\"serialize\"));\n",
                var_name
            ));
            out.push_str("        }\n");
        } else {
            out.push_str(&format!(
                "        vars.insert(\"{}\".to_string(), serde_json::to_value({}).expect(\"serialize\"));\n",
                var_name, rust_name
            ));
        }
    }
    out
}

fn render_variable_defs(args: &[InputValue<String>]) -> String {
    let mut defs = Vec::new();
    for arg in args {
        let gql_type = format_gql_type(&arg.value_type);
        defs.push(format!("${}: {}", arg.name, gql_type));
    }
    defs.join(", ")
}

fn render_field_args(args: &[InputValue<String>]) -> String {
    if args.is_empty() {
        return String::new();
    }

    let mut parts = Vec::new();
    for arg in args {
        parts.push(format!("{}: ${}", arg.name, arg.name));
    }
    format!("({})", parts.join(", "))
}

fn selection_for_field(field: &Field<String>, ctx: &SchemaContext) -> String {
    let base = base_type_name(&field.field_type);
    if is_scalar_type(&base) || ctx.enums.contains(&base) || ctx.scalars.contains(&base) {
        return String::new();
    }

    let mut stack = BTreeSet::new();
    let selection = selection_for_type(&base, ctx, &mut stack, 0);
    if selection.is_empty() {
        String::new()
    } else {
        format!(" {}", selection)
    }
}

fn selection_for_type(
    type_name: &str,
    ctx: &SchemaContext,
    stack: &mut BTreeSet<String>,
    depth: usize,
) -> String {
    if depth > 3 {
        return "{ __typename }".to_string();
    }

    if stack.contains(type_name) {
        if let Some(TypeDefinition::Object(obj)) = ctx.types.get(type_name) {
            if obj.fields.iter().any(|f| f.name == "id") {
                return "{ id }".to_string();
            }
        }
        return "{ __typename }".to_string();
    }

    stack.insert(type_name.to_string());

    let mut fields = Vec::new();
    if let Some(TypeDefinition::Object(obj)) = ctx.types.get(type_name) {
        for field in &obj.fields {
            if has_required_args(field) {
                continue;
            }
            let field_base = base_type_name(&field.field_type);
            if is_scalar_type(&field_base)
                || ctx.enums.contains(&field_base)
                || ctx.scalars.contains(&field_base)
            {
                fields.push(field.name.clone());
                continue;
            }

            if ctx.objects.contains(&field_base) {
                let nested = selection_for_type(&field_base, ctx, stack, depth + 1);
                fields.push(format!("{} {}", field.name, nested));
                continue;
            }

            if ctx.unions.contains(&field_base) {
                fields.push(format!("{} {{ __typename }}", field.name));
                continue;
            }
        }
    }

    stack.remove(type_name);

    if fields.is_empty() {
        String::new()
    } else {
        format!("{{ {} }}", fields.join(" "))
    }
}

fn base_type_name(ty: &Type<String>) -> String {
    match ty {
        Type::NamedType(name) => name.clone(),
        Type::NonNullType(inner) => base_type_name(inner),
        Type::ListType(inner) => base_type_name(inner),
    }
}

fn has_required_args(field: &Field<String>) -> bool {
    field
        .arguments
        .iter()
        .any(|arg| matches!(arg.value_type, Type::NonNullType(_)) && arg.default_value.is_none())
}

fn is_optional(ty: &Type<String>) -> bool {
    !matches!(ty, Type::NonNullType(_))
}

fn is_scalar_type(name: &str) -> bool {
    matches!(
        name,
        "String" | "Int" | "Float" | "Boolean" | "ID" | "DateTime" | "BigInt" | "GenericScalar"
    )
}

fn rust_type(ty: &Type<String>, ctx: &SchemaContext, input: bool) -> String {
    match ty {
        Type::NonNullType(inner) => rust_type_nonnull(inner, ctx, input, false),
        _ => format!("Option<{}>", rust_type_nonnull(ty, ctx, input, false)),
    }
}

fn rust_type_nonnull(ty: &Type<String>, ctx: &SchemaContext, input: bool, in_list: bool) -> String {
    match ty {
        Type::ListType(inner) => format!("Vec<{}>", rust_type_nonnull(inner, ctx, input, true)),
        Type::NonNullType(inner) => rust_type_nonnull(inner, ctx, input, in_list),
        Type::NamedType(name) => match name.as_str() {
            "String" | "ID" | "DateTime" => "String".to_string(),
            "Int" => "i64".to_string(),
            "Float" => "f64".to_string(),
            "Boolean" => "bool".to_string(),
            "BigInt" => "i64".to_string(),
            "GenericScalar" => "serde_json::Value".to_string(),
            _ => {
                if ctx.enums.contains(name)
                    || ctx.inputs.contains(name)
                    || ctx.scalars.contains(name)
                {
                    name.clone()
                } else if ctx.objects.contains(name) {
                    if input || in_list {
                        name.clone()
                    } else {
                        format!("Box<{}>", name)
                    }
                } else if ctx.unions.contains(name) {
                    name.to_string()
                } else {
                    "serde_json::Value".to_string()
                }
            }
        },
    }
}

fn format_gql_type(ty: &Type<String>) -> String {
    match ty {
        Type::NamedType(name) => name.clone(),
        Type::NonNullType(inner) => format!("{}!", format_gql_type(inner)),
        Type::ListType(inner) => format!("[{}]", format_gql_type(inner)),
    }
}

fn to_rust_ident(name: &str) -> String {
    let mut out = String::new();
    let mut upper = true;
    for ch in name.chars() {
        if ch == '_' || ch == '-' {
            upper = true;
            continue;
        }
        if upper {
            out.extend(ch.to_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    match out.as_str() {
        "Self" | "Type" | "Box" | "Result" => format!("{}Type", out),
        _ => out,
    }
}

fn to_rust_field(name: &str) -> String {
    let mut out = String::new();
    for (idx, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            if idx > 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }

    if is_rust_keyword(&out) {
        format!("r#{}", out)
    } else {
        out
    }
}

struct MethodArgs {
    signature: String,
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
    )
}
