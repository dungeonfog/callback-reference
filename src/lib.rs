use uuid::Uuid;

pub struct CallbackReference {
    handle: Uuid,
    drop_handler: Option<Box<dyn FnOnce(Uuid) + 'static + Send>>,
}

impl CallbackReference {
    pub fn new(handle: Uuid, drop_handler: impl FnOnce(Uuid) + 'static + Send) -> Self {
        Self {
            handle,
            drop_handler: Some(Box::new(drop_handler)),
        }
    }
}

impl std::cmp::PartialEq for CallbackReference {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl std::cmp::Eq for CallbackReference {}

impl std::hash::Hash for CallbackReference {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl std::borrow::Borrow<Uuid> for CallbackReference {
    fn borrow(&self) -> &Uuid {
        &self.handle
    }
}

impl Drop for CallbackReference {
    fn drop(&mut self) {
        self.drop_handler.take().unwrap()(self.handle)
    }
}

use std::fmt;
impl fmt::Debug for CallbackReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CallbackReference")
         .field("handle", &self.handle)
         .finish()
    }
}
