pub struct CallbackReference<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug {
    handle: HANDLE,
    drop_handler: Option<Box<dyn FnOnce() + 'static>>,
}

impl<HANDLE> CallbackReference<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug {
    pub fn new(handle: HANDLE, drop_handler: impl FnOnce() + 'static) -> Self {
        Self {
            handle,
            drop_handler: Some(Box::new(drop_handler)),
        }
    }
}

impl<HANDLE> std::cmp::PartialEq for CallbackReference<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl<HANDLE> std::cmp::Eq for CallbackReference<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug {}

impl<HANDLE> std::hash::Hash for CallbackReference<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl<HANDLE> std::borrow::Borrow<HANDLE> for CallbackReference<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug {
    fn borrow(&self) -> &HANDLE {
        &self.handle
    }
}

impl<HANDLE> Drop for CallbackReference<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug {
    fn drop(&mut self) {
        self.drop_handler.take().unwrap()()
    }
}

use std::fmt;
impl<HANDLE> fmt::Debug for CallbackReference<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CallbackReference")
         .field("handle", &self.handle)
         .finish()
    }
}
