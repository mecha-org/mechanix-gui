use crate::service::{ExternalSearchResult, ExternalService, UpsertMetadata};
use log::{error, info};
use tantivy::schema::document::CompactDocValue;
use tantivy::schema::{Field, Schema, Value, STORED, STRING, TEXT};
use tantivy::{doc, Document, TantivyDocument};

#[derive(Debug, Clone)]
pub struct MetadataFields {
    pub source: Field,
    pub unique_id: Field,
    pub uri: Field,
    pub title: Field,
    pub subtitle: Field,
    pub description: Field,
    pub keywords: Field,
    pub icon: Field,
    pub thumbnail: Field,
    pub last_modified: Field,
    pub content: Field,
    pub source_entry_path: Field,
}

pub struct SchemaBundle {
    pub schema: Schema,
    pub fields: MetadataFields,
}
impl ExternalService {

    //TODO: Unused code, need to refactor so that we can use same in indexer.rs

    // pub fn make_doc(&self, meta: &UpsertMetadata) -> TantivyDocument {
    //     let fields = &self.fields;
    //
    //     doc!(
    //         fields.source => meta.source,
    //         fields.uri => meta.uri,
    //         fields.title => meta.title,
    //         fields.subtitle => meta.subtitle,
    //         fields.description => meta.description,
    //         fields.icon => meta.icon,
    //         fields.content => meta.content.clone().unwrap_or_default(),
    //         fields.last_modified => meta.last_modified,
    //         fields.unique_id => meta.unique_id,
    //         // fields.keywords => meta.keywords.clone(),
    //         fields.thumbnail => meta.thumbnail,
    //         fields.source_entry_path => meta.source_entry_path
    //     )
    // }

    // Build schema and pre-resolve field handles in one place
    pub(crate) fn build_schema() -> anyhow::Result<SchemaBundle> {
        let mut b = Schema::builder();
        // retrievable & searchable
        b.add_text_field("title", TEXT | STORED);
        b.add_text_field("subtitle", TEXT | STORED);
        b.add_text_field("description", TEXT | STORED);
        b.add_text_field("content", TEXT);
        b.add_text_field("keywords", TEXT);

        // identifiers & metadata
        b.add_text_field("source", STRING | STORED);
        b.add_text_field("uri", STRING | STORED);
        b.add_text_field("source_entry_path", STRING);
        b.add_text_field("unique_id", STRING | STORED);
        b.add_u64_field("last_modified", STORED);

        // media
        b.add_text_field("icon", STORED);
        b.add_text_field("thumbnail", STORED);
        let schema = b.build();

        // Reuse the existing helper to resolve the fields we care about in results
        let fields = match Self::build_fields(&schema) {
            Ok(fields) => fields,
            Err(e) => {
                error!("Failed to build fields: {}", e);
                return Err(e);
            }
        };

        Ok(SchemaBundle { schema, fields })
    }

    pub(crate) fn build_fields(schema: &Schema) -> anyhow::Result<MetadataFields> {
        Ok(MetadataFields {
            source: schema.get_field("source")?,
            unique_id: schema.get_field("unique_id")?,
            uri: schema.get_field("uri")?,
            title: schema.get_field("title")?,
            subtitle: schema.get_field("subtitle")?,
            description: schema.get_field("description")?,
            keywords: schema.get_field("keywords")?,
            icon: schema.get_field("icon")?,
            thumbnail: schema.get_field("thumbnail")?,
            last_modified: schema.get_field("last_modified")?,
            content: schema.get_field("content")?,
            source_entry_path: schema.get_field("source_entry_path")?,
        })
    }

    // Map a tantivy document into our result struct using pre-resolved fields.
    pub(crate) fn map_doc(&self, doc: &TantivyDocument) -> ExternalSearchResult {
        let f = &self.fields;
        ExternalSearchResult {
            source: get_text(doc, f.source).unwrap_or_default(),
            uri: get_text(doc, f.uri).unwrap_or_default(),
            title: get_text(doc, f.title).unwrap_or_default(),
            icon: get_text(doc, f.icon).unwrap_or_default(),
            thumbnail: get_text(doc, f.thumbnail).unwrap_or_default(),
            last_modified: get_u64(doc, f.last_modified).unwrap_or(0),
            content: get_text(doc, f.content).unwrap_or_default(),
            unique_id: get_text(doc, f.unique_id).unwrap_or_default(),
            keywords: get_vec(doc, f.keywords),
            score: 0.0,
        }
    }
}

fn join(values: &Vec<CompactDocValue>) -> String {
    values
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(";")
}

fn get_vec(doc: &TantivyDocument, f: Field) -> Vec<String> {
    doc.get_all(f)
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect()
}
fn get_text(doc: &TantivyDocument, f: Field) -> Option<String> {
    doc.get_all(f)
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .next() // if you only expect single-valued fields
}

fn get_u64(doc: &TantivyDocument, f: Field) -> Option<u64> {
    doc.get_first(f).and_then(|v| v.as_u64())
}