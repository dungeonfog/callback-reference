use std::fmt;

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

impl<HANDLE> fmt::Debug for CallbackReference<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CallbackReference")
         .field("handle", &self.handle)
         .finish()
    }
}

pub struct CallbackReferenceSend<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug + Send {
    handle: HANDLE,
    drop_handler: Option<Box<dyn FnOnce() + 'static + Send>>,
}

impl<HANDLE> CallbackReferenceSend<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug + Send {
    pub fn new(handle: HANDLE, drop_handler: impl FnOnce() + 'static + Send) -> Self {
        Self {
            handle,
            drop_handler: Some(Box::new(drop_handler)),
        }
    }
}

impl<HANDLE> std::cmp::PartialEq for CallbackReferenceSend<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug + Send {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl<HANDLE> std::cmp::Eq for CallbackReferenceSend<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug + Send {}

impl<HANDLE> std::hash::Hash for CallbackReferenceSend<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug + Send {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl<HANDLE> std::borrow::Borrow<HANDLE> for CallbackReferenceSend<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug + Send {
    fn borrow(&self) -> &HANDLE {
        &self.handle
    }
}

impl<HANDLE> Drop for CallbackReferenceSend<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug + Send {
    fn drop(&mut self) {
        self.drop_handler.take().unwrap()()
    }
}

impl<HANDLE> fmt::Debug for CallbackReferenceSend<HANDLE> where HANDLE: Eq + std::hash::Hash + std::fmt::Debug + Send {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CallbackReferenceSend")
         .field("handle", &self.handle)
         .finish()
    }
}
