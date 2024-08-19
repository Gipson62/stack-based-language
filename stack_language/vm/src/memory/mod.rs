pub struct Memory {
    mem: Vec<Object>,
    pub(crate) free: ObjectIndex,
}

#[derive(Clone, Copy, Debug)]
pub struct ObjectIndex {
    pub(crate) idx: u64,
}

impl ObjectIndex {
    pub fn new(i: u64) -> ObjectIndex {
        ObjectIndex { idx: i }
    }
}

impl Memory {
    pub(crate) fn new(space: usize) -> Self {
        Self {
            free: ObjectIndex::new(0),
            mem: (0..space)
                .map(|x| Object::Free {
                    next: ObjectIndex::new(((x + 1) % space) as u64),
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Object {
    Integer(i64),
    Float(f64),
    String(String),
    
    Free { next: ObjectIndex },
}

pub struct Structure {
    fields: Vec<Object>
}
