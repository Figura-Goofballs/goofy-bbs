//! See [LockView].

use std::mem::replace;
use std::sync::{ RwLock, RwLockReadGuard, RwLockWriteGuard };
use cursive::{ View, view::ViewWrapper };

// FIX(PoolloverNathan): I'm certain that this already exists but I can't find it
/// A LockView is a [View] wrapper that can be atomically replaced (or otherwise modified) at
/// runtime. This allows easy multi-screen layouts.
pub struct LockView<V: View>(pub RwLock<V>);
impl<V: View> LockView<V> {
    /// Returns a [RwLockReadGuard] to the contained view.
    /// # Panics
    /// If the lock is poisoned, this method will panic.
    pub fn get(&self) -> RwLockReadGuard<V> {
        self.0.read().unwrap()
    }
    /// Returns a [RwLockWriteGuard] to the contained view.
    /// # Panics
    /// If the lock is poisoned, this method will panic. Prefer using the non-panicking atomic
    /// methods such as [replace] instead.
    pub fn get_mut(&self) -> RwLockWriteGuard<V> {
        self.0.write().unwrap()
    }
    /// Replaces the internal view with a new view.
    /// # Panics
    /// If the lock is poisoned, this method will panic. However, this method should never itself
    /// panic.
    pub fn replace(&self, new_view: V) -> V {
        replace(&mut *self.get_mut(), new_view)
    }
}
impl<V: View> ViewWrapper for LockView<V> {
    type V = V;
    fn with_view<F, R>(&self, f: F) -> Option<R> where F: FnOnce(&V) -> R {
        Some(f(self.get()))
    }
    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R> where F: FnOnce(&mut V) -> R {
        Some(f(self.get()))
    }
    fn into_inner(self) -> Result<V, Self> {
        Ok(self.0.into_inner())
    }
}
