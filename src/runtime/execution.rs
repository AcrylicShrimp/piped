use super::imported_pipeline::ImportedPipeline;
use super::sub_execution::SubExecution;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct Execution {
	imported_pipeline_map: Mutex<HashMap<PathBuf, Arc<ImportedPipeline>>>,
}

impl Execution {
	pub fn new() -> Execution {
		Execution {
			imported_pipeline_map: Mutex::new(HashMap::new()),
		}
	}

	pub fn execute(self, entry_pipeline: ImportedPipeline) {
		let execution = Arc::new(self);
		let entry_pipeline = Arc::new(entry_pipeline);

		(*execution.imported_pipeline_map.lock().unwrap())
			.insert(entry_pipeline.path().clone(), entry_pipeline.clone());

		SubExecution::new(execution).execute(&entry_pipeline);
	}

	pub fn import(&self, path: PathBuf) -> Result<Arc<ImportedPipeline>, String> {
		let imported_pipeline_map = &mut *self.imported_pipeline_map.lock().unwrap();

		if imported_pipeline_map.contains_key(&path) {
			return Err("A pipeline with this name is already exists.".to_owned());
		}

		let pipeline = Arc::new(ImportedPipeline::import(&path)?);
		imported_pipeline_map.insert(path, pipeline.clone());

		Ok(pipeline)
	}
}
