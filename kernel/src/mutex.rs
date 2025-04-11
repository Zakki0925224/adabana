use crate::{asm, error::Result};
use core::{
    cell::{SyncUnsafeCell, UnsafeCell},
    ops::{Deref, DerefMut},
};

pub struct Mutex<T> {
    value: SyncUnsafeCell<T>,
    locked: UnsafeCell<bool>, // cannot use AtomicBool
}

impl<T: Sized> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: SyncUnsafeCell::new(value),
            locked: UnsafeCell::new(false),
        }
    }

    pub fn try_lock(&self) -> Result<MutexGuard<T>> {
        asm::disabled_int(|| {
            if !self.locked() {
                self.set_locked(true);
                return Ok(unsafe { MutexGuard::new(self, &self.value) });
            }

            Err("Mutex is already locked".into())
        })
    }

    fn locked(&self) -> bool {
        unsafe { *self.locked.get() }
    }

    fn set_locked(&self, locked: bool) {
        unsafe {
            *self.locked.get() = locked;
        }
        assert_eq!(self.locked(), locked);
    }
}

unsafe impl<T> Sync for Mutex<T> {}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
    value: &'a mut T,
}

impl<'a, T> MutexGuard<'a, T> {
    unsafe fn new(mutex: &'a Mutex<T>, value: &'a SyncUnsafeCell<T>) -> Self {
        Self {
            mutex,
            value: &mut *value.get(),
        }
    }
}

unsafe impl<'a, T> Sync for MutexGuard<'a, T> {}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        asm::disabled_int(|| self.mutex.set_locked(false));
    }
}
