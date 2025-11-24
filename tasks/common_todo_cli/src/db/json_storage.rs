// use std::{
//     fs::File,
//     io::{BufReader, Write},
//     path::PathBuf,
// };

// use serde::{Deserialize, Serialize};

// use crate::{
//     db::{Storage, storage::StorageError},
//     model::task::Task,
// };

// const FIRST_ITEM: u32 = 1;

// #[derive(Serialize, Deserialize)]
// struct InnerDB {
//     tasks: Vec<Task>,
//     next_id: u32,
// }

// pub struct JsonStorage {
//     file_path: PathBuf,
// }

// impl JsonStorage {
//     pub fn new() -> Result<Self, StorageError> {
//         let default_path = PathBuf::from("common_todo_db.json");

//         if !default_path.exists() {
//             let inner_db = InnerDB {
//                 tasks: vec![],
//                 next_id: FIRST_ITEM,
//             };
//             let data = serde_json::to_string_pretty(&inner_db)?;
//             let mut file = File::create(&default_path)?;
//             file.write_all(data.as_bytes())?;
//         }

//         Ok(Self {
//             file_path: default_path,
//         })
//     }

//     fn load_inner_db(&self) -> Result<InnerDB, StorageError> {
//         let path = self.file_path.clone();
//         let file = File::open(path)?;
//         let buf_reader = BufReader::new(file);
//         let data: InnerDB = serde_json::from_reader(buf_reader)?;
//         Ok(data)
//     }

//     fn save_inner_db(&self, inner_db: &InnerDB) -> Result<(), StorageError> {
//         let data = serde_json::to_string_pretty(inner_db)?;
//         let mut file = File::create(self.file_path.clone())?;
//         file.write_all(data.as_bytes())?;
//         Ok(())
//     }
// }

// impl Storage for JsonStorage {
//     fn create(&self, value: String) -> Result<Task, StorageError> {
//         let mut inner_db = self.load_inner_db()?;
//         let id = inner_db.next_id;

//         let task = Task::new(id, value);
//         inner_db.tasks.push(task.clone());
//         inner_db.next_id += 1;

//         self.save_inner_db(&inner_db)?;

//         Ok(task)
//     }

//     fn read(&self, id: u32) -> Result<Task, StorageError> {
//         let inner_db = self.load_inner_db()?;

//         let task = inner_db
//             .tasks
//             .iter()
//             .find(|task| task.id == id)
//             .cloned()
//             .ok_or(StorageError::RecordNotFound { id })?;

//         Ok(task)
//     }

//     fn update(&self, id: u32, value: String) -> Result<Task, StorageError> {
//         let mut inner_db = self.load_inner_db()?;

//         let task = inner_db
//             .tasks
//             .iter_mut()
//             .find(|task| task.id == id)
//             .ok_or(StorageError::RecordNotFound { id })?;
//         task.value = value;

//         let result_task = task.clone();

//         self.save_inner_db(&inner_db)?;
//         Ok(result_task)
//     }

//     fn delete(&self, id: u32) -> Result<(), StorageError> {
//         let mut inner_db = self.load_inner_db()?;

//         let task_index = inner_db
//             .tasks
//             .iter()
//             .position(|task| task.id == id)
//             .ok_or(StorageError::RecordNotFound { id })?;

//         inner_db.tasks.remove(task_index);

//         self.save_inner_db(&inner_db)?;
//         Ok(())
//     }

//     fn list(&self) -> Result<Vec<Task>, StorageError> {
//         let inner_db = self.load_inner_db()?;
//         let tasks = inner_db.tasks;
//         Ok(tasks)
//     }

//     fn mark_ready_or_not(&self, id: u32, is_ready: bool) -> Result<Task, StorageError> {
//         let mut inner_db = self.load_inner_db()?;

//         let task = inner_db
//             .tasks
//             .iter_mut()
//             .find(|task| task.id == id)
//             .ok_or(StorageError::RecordNotFound { id })?;
//         task.is_ready = is_ready;

//         let result_task = task.clone();

//         self.save_inner_db(&inner_db)?;
//         Ok(result_task)
//     }
// }
