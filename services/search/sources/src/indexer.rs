use crate::service::{IndexCmd, UpsertMetadata};
use crate::utils::MetadataFields;
use log::{error, info, warn};
use tantivy::{doc, IndexWriter, TantivyDocument, Term};
use tokio::sync::mpsc;

pub struct Indexer {
    writer: IndexWriter,
    fields: MetadataFields,
    commit_interval: std::time::Duration,
}

impl Indexer {
    pub fn new(writer: IndexWriter, fields: MetadataFields) -> Self {
        Self {
            writer,
            fields,
            commit_interval: std::time::Duration::from_secs(2),
        }
    }

    fn make_doc(&self, m: &UpsertMetadata) -> TantivyDocument {
        doc!(
            self.fields.source => m.source.clone(),
            self.fields.uri => m.uri.clone(),
            self.fields.title => m.title.clone(),
            self.fields.subtitle => m.subtitle.clone(),
            self.fields.description => m.description.clone(),
            self.fields.icon => m.icon.clone(),
            self.fields.content => m.content.clone().unwrap_or_default(),
            self.fields.last_modified => m.last_modified,
            self.fields.unique_id => m.unique_id.clone(),
            self.fields.source_entry_path => m.source_entry_path.clone(),
            self.fields.keywords => m.keywords.join(" "),
            self.fields.thumbnail => m.thumbnail.clone()
        )
    }

    // Apply pending deletes and upserts to the writer.
    fn apply(&mut self, upserts: &mut Vec<UpsertMetadata>, deletes: &mut Vec<Term>) {
        if !deletes.is_empty() {
            for term in deletes.drain(..) {
                self.writer.delete_term(term);
            }
        }
        if !upserts.is_empty() {
            for m in upserts.drain(..) {
                if let Err(e) = self.writer.add_document(self.make_doc(&m)) {
                    error!("add_document failed: {}", e);
                }
            }
        }
    }

    // Commit if there was any activity.
    fn commit_if_needed(&mut self, had_activity: bool) {
        if had_activity {
            if let Err(e) = self.writer.commit() {
                error!("commit failed: {}", e);
            } else {
                info!("index commit complete");
            }
        }
    }

    pub async fn run(mut self, mut rx: mpsc::Receiver<IndexCmd>) {
        let mut pending_upserts: Vec<UpsertMetadata> = Vec::new();
        let mut pending_deletes: Vec<Term> = Vec::new();
        let mut tick = tokio::time::interval(self.commit_interval);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        // Helper closures replaced with &mut self methods to avoid borrow checker issues

        loop {
            tokio::select! {
                biased;
                maybe_cmd = rx.recv() => {
                    match maybe_cmd {
                        Some(IndexCmd::Upsert(batch)) => {
                            pending_upserts.extend(batch);
                            self.apply(&mut pending_upserts, &mut pending_deletes);
                            self.commit_if_needed(true);
                        }
                        Some(IndexCmd::RemoveByUniqueIds(ids, need_commit)) => {
                            ids.iter().for_each(|id| {
                                let t = Term::from_field_text(self.fields.unique_id, &id);
                                pending_deletes.push(t);
                            });
                            self.apply(&mut pending_upserts, &mut pending_deletes);
                            self.commit_if_needed(need_commit);
                        }
                        Some(IndexCmd::RemoveByPath(path)) => {
                            let p = path;
                            info!("Removing path: {}", p);
                            let t = Term::from_field_text(self.fields.source_entry_path, &p);
                            pending_deletes.push(t);
                            self.apply(&mut pending_upserts, &mut pending_deletes);
                            self.commit_if_needed(true);
                        }
                        Some(IndexCmd::Flush) => {
                            let had = !pending_upserts.is_empty() || !pending_deletes.is_empty();
                            self.apply(&mut pending_upserts, &mut pending_deletes);
                            self.commit_if_needed(had);
                        }
                        Some(IndexCmd::Shutdown) => {
                            warn!("Indexer: shutdown received, flushing...");
                            let had = !pending_upserts.is_empty() || !pending_deletes.is_empty();
                            self.apply(&mut pending_upserts, &mut pending_deletes);
                            self.commit_if_needed(had);
                            break;
                        }
                        None => {
                            warn!("Indexer: channel closed, exiting");
                            break;
                        }
                    }
                }
                _ = tick.tick() => {
                    // periodic commit to reduce crash window and make data visible
                    let had = !pending_upserts.is_empty() || !pending_deletes.is_empty();
                    if had {
                        self.apply(&mut pending_upserts, &mut pending_deletes);
                        self.commit_if_needed(true);
                    }
                }
            }
        }
    }
}
