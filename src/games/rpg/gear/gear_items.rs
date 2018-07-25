use super::errors::GearParseError;
use super::items::LegalGear;
use super::GearID;
use ron::de::from_reader;
use std::collections::HashMap;
use std::fs;

pub struct GearItems {
    path: Option<::std::path::PathBuf>,
    data: HashMap<GearID, LegalGear>,
}

impl GearItems {
    pub fn new() -> GearItems {
        GearItems {
            path: None,
            data: HashMap::default(),
        }
    }
    pub fn get(&self, id: &GearID) -> Result<&LegalGear, GearParseError> {
        match self.data.get(id) {
            Some(l) => Ok(l),
            None => Err(GearParseError::DoesNotExist { id: *id }),
        }
    }
    pub fn from_dir(p: ::std::path::PathBuf) -> GearItems {
        let mut g = GearItems::new();
        g.load_dir(p);
        g
    }
    pub fn load_dir(&mut self, p: ::std::path::PathBuf) {
        let mut id = 0;
        match fs::read_dir(&p) {
            Ok(entries) => {
                self.path = Some(p);
                entries
                    .into_iter()
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|e| Some(e.path()))
                            .and_then(
                                |p: ::std::path::PathBuf| {
                                    if p.ends_with(".ron") {
                                        Some(p)
                                    } else {
                                        None
                                    }
                                },
                            )
                            .and_then(|p| fs::File::open(p).ok())
                            .and_then(|f: fs::File| from_reader(f).ok())
                            .and_then(|val| {
                                self.data.insert(id, val).and_then(|_| {
                                    id += 1;
                                    None
                                })
                            })
                    })
                    .collect::<Vec<()>>()
            }
            Err(_) => vec![],
        };
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn path(&self) -> Option<&::std::path::Path> {
        match self.path {
            Some(ref p) => Some(p.as_path()),
            None => None,
        }
    }
}
