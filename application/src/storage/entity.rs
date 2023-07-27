use uuid::Uuid;

use super::engine::Engine;
use super::engine::fs::Engine as FSEngine;

pub struct Storage {
    id: Uuid,
    name: String,
    settings: Settings,
}

impl Storage {
    pub fn new(name: String, settings: Settings) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            settings,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // pub fn connect_box_dyn<T>(&self) -> Result<ConnectedStorageBoxDyn<T>, anyhow::Error> {
    //     let engine = match self.settings {
    //         Settings::FS { base_path } => FSEngine::new(base_path)?,
    //     };

    //     Ok(ConnectedStorageBoxDyn {
    //         engine,
    //     })
    // }

    pub fn connect_generic<E: Engine>(&self) -> Result<ConnectedStorageGeneric<E>, anyhow::Error> {
        let engine = match self.settings {
            Settings::FS { base_path } => FSEngine::new(base_path)?,
        };

        Ok(ConnectedStorageGeneric {
            engine,
        })
    }
}

pub enum Settings {
    FS {
        base_path: String,
    }
}

// struct ConnectedStorageBoxDyn<Path> {
//     engine: Box<dyn Engine<Path = Path>>
// }

struct ConnectedStorageGeneric<E: Engine> {
    engine: E,
}