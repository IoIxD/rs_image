// cbindgen doesn't like make_thin_trait so we have to expand the result here

use std::{
    ffi::c_void,
    iter::{
        Enumerate, Filter, FilterMap, FlatMap, Fuse, Inspect, Map, MapWhile, Peekable, Scan, Skip,
        SkipWhile, StepBy, Take, TakeWhile, Zip,
    },
};

use image::Pixels;

use crate::{iter::RawIterator, PixelResult, Rgba};

pub trait ThinIterator {
    fn next(&mut self) -> *mut c_void;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash)]
pub struct ThinIteratorVtable {
    pub next: unsafe fn(*mut ::core::ffi::c_void) -> *mut c_void,
    pub drop: unsafe fn(*mut ::core::ffi::c_void),
}

#[repr(C)]
pub struct __ThinTraitObjectMacro_ReprForThinIterator<
    __ThinTraitObjectMacro_ReprGeneric0: ThinIterator,
> {
    __thintraitobjectmacro_repr_vtable: &'static ThinIteratorVtable,
    __thintraitobjectmacro_repr_value: __ThinTraitObjectMacro_ReprGeneric0,
}
impl<__ThinTraitObjectMacro_ReprGeneric0: ThinIterator>
    __ThinTraitObjectMacro_ReprForThinIterator<__ThinTraitObjectMacro_ReprGeneric0>
{
    const __THINTRAITOBJECTMACRO_VTABLE: ThinIteratorVtable = ThinIteratorVtable {
        next: Self::__thintraitobjectmacro_thunk_next,
        drop: Self::__thintraitobjectmacro_repr_drop,
    };
    fn __thintraitobjectmacro_repr_create(
        __thintraitobjectmacro_arg0: __ThinTraitObjectMacro_ReprGeneric0,
    ) -> *mut ThinIteratorVtable {
        ::std::boxed::Box::into_raw(::std::boxed::Box::new(Self {
            __thintraitobjectmacro_repr_vtable: &Self::__THINTRAITOBJECTMACRO_VTABLE,
            __thintraitobjectmacro_repr_value: __thintraitobjectmacro_arg0,
        })) as *mut _
    }
    unsafe fn __thintraitobjectmacro_repr_drop(
        __thintraitobjectmacro_arg0: *mut ::core::ffi::c_void,
    ) {
        let _ = ::std::boxed::Box::from_raw(
            __thintraitobjectmacro_arg0
                as *mut __ThinTraitObjectMacro_ReprForThinIterator<
                    __ThinTraitObjectMacro_ReprGeneric0,
                >,
        );
    }
    unsafe fn __thintraitobjectmacro_thunk_next(
        __thintraitobjectmacro_arg0: *mut ::core::ffi::c_void,
    ) -> *mut c_void {
        (*(__thintraitobjectmacro_arg0
            as *mut __ThinTraitObjectMacro_ReprForThinIterator<
                __ThinTraitObjectMacro_ReprGeneric0,
            >))
            .__thintraitobjectmacro_repr_value
            .next()
    }
}
#[repr(transparent)]
pub struct BoxedThinIterator<'inner>(
    ::core::ptr::NonNull<ThinIteratorVtable>,
    ::core::marker::PhantomData<&'inner ()>,
);
impl<'inner> BoxedThinIterator<'inner> {
    /// Constructs a boxed thin trait object from a type implementing the trait.
    #[inline]
    pub fn new<T: ThinIterator + Sized + 'inner>(val: T) -> Self {
        unsafe {
            Self::from_raw(
                __ThinTraitObjectMacro_ReprForThinIterator::__thintraitobjectmacro_repr_create(val)
                    as *mut _,
            )
        }
    }
    /// Creates a thin trait object directly from a raw pointer to its vtable.
    ///
    /// # Safety
    /// This constructor, by its nature, is hugely unsafe and should be avoided when possible. The following invariants must be upheld:
    /// - The pointer must not be null and must point to a valid thin trait object as expected by its vtable which is not uninitialized;
    /// - The function pointers in the vtable must not be null and must point to valid functions with correct ABI and signature;
    /// - The function pointers must have the same safety contract as implied and not a stronger one: only cause UB if the vtable pointer passed to them is invalid or, if those are unsafe in the trait itself, cause UB if the safety contract in their declarations is violated;
    /// - If the trait is unsafe, the function pointers must follow the trait's contract for valid implementations;
    /// - The pointer was not returned by [`as_raw`] which was called on an object which was not put into [`ManuallyDrop`] or consumed by [`mem::forget`], otherwise undefined behavior will be invoked when both are dropped.
    ///
    /// [`as_raw`]: #method.as_raw " "
    /// [`ManuallyDrop`]: https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html " "
    /// [`mem::forget`]: https://doc.rust-lang.org/std/mem/fn.forget.html " "
    #[inline]
    pub const unsafe fn from_raw(ptr: *mut ()) -> Self {
        Self(
            ::core::ptr::NonNull::new_unchecked(ptr as *mut _),
            ::core::marker::PhantomData,
        )
    }
    /// Extracts the contained pointer to the trait object.
    ///
    /// Unlike [`into_raw`], ownership of the pointer is not released, and as such will be dropped normally. Unless the original copy is removed via [`mem::forget`] or [`ManuallyDrop`], calling [`from_raw`] and then dropping will cause undefined behavior.
    ///
    /// [`into_raw`]: #method.into_raw " "
    /// [`from_raw`]: #method.from_raw " "
    /// [`ManuallyDrop`]: https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html " "
    /// [`mem::forget`]: https://doc.rust-lang.org/std/mem/fn.forget.html " "
    /*#[inline]
    pub const fn as_raw(&self) -> *mut () {
        self.0.as_ptr() as *mut ()
    }
    /// Releases ownership of the trait object, returning the contained pointer. It is the caller's responsibility to drop the trait object at a later time using [`from_raw`].
    ///
    /// For a version which does not release ownership, see [`as_raw`].
    ///
    /// [`from_raw`]: #method.from_raw " "
    /// [`as_raw`]: #method.as_raw " "
    #[inline]
    pub fn into_raw(self) -> *mut () {
        let pointer = self.as_raw();
        ::core::mem::forget(self);
        pointer
    }*/
    /// Retrieves the raw vtable of the contained trait object.
    pub fn vtable(&self) -> &ThinIteratorVtable {
        unsafe { &*(self.0.as_ptr() as *mut &'static ThinIteratorVtable) }
    }
}
#[allow(clippy::ref_in_deref)]
impl ThinIterator for BoxedThinIterator<'_> {
    fn next(&mut self) -> *mut c_void {
        unsafe { ((self.vtable()).next)(self.0.as_ptr() as *mut _) }
    }
}
impl ::core::ops::Drop for BoxedThinIterator<'_> {
    fn drop(&mut self) {
        unsafe { (self.vtable().drop)(self.0.as_ptr() as *mut ::core::ffi::c_void) }
    }
}

impl Iterator for dyn ThinIterator {
    type Item = *mut c_void;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { ThinIterator::next(self).as_mut() } {
            Some(a) => Some(a),
            None => None,
        }
    }
}

impl ThinIterator for Pixels<'_, image::DynamicImage> {
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(PixelResult {
                x: r.0,
                y: r.1,
                color: Rgba {
                    r: r.2 .0[0],
                    g: r.2 .0[1],
                    b: r.2 .0[2],
                    a: r.2 .0[3],
                },
            })) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl Iterator for BoxedThinIterator<'_> {
    type Item = *mut c_void;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { ThinIterator::next(self).as_mut() } {
            Some(a) => Some(a),
            None => None,
        }
    }
}

impl ThinIterator for StepBy<RawIterator> {
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl ThinIterator for std::iter::Chain<RawIterator, RawIterator> {
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl ThinIterator for Zip<RawIterator, RawIterator> {
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl ThinIterator for Enumerate<RawIterator> {
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl ThinIterator for Peekable<RawIterator> {
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl ThinIterator for Skip<RawIterator> {
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl ThinIterator for Take<RawIterator> {
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl ThinIterator for Fuse<RawIterator> {
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl<I, F> ThinIterator for Map<I, F>
where
    I: Iterator,
    F: FnMut(I::Item),
{
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl<I, U, F> ThinIterator for FlatMap<I, U, F>
where
    I: Iterator,
    U: IntoIterator,
    F: FnMut(I::Item) -> U,
{
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl<I, F> ThinIterator for Filter<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl<I, F> ThinIterator for FilterMap<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> Option<I>,
{
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl<I, F> ThinIterator for SkipWhile<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl<I, F> ThinIterator for TakeWhile<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl<I, F> ThinIterator for MapWhile<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> Option<I>,
{
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl<B, I, F, St> ThinIterator for Scan<I, St, F>
where
    I: Iterator,
    F: FnMut(&mut St, <I as Iterator>::Item) -> Option<B>,
{
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl<I, F> ThinIterator for Inspect<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item),
{
    fn next(&mut self) -> *mut c_void {
        match Iterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}

impl ThinIterator for &mut RawIterator {
    fn next(&mut self) -> *mut c_void {
        match RawIterator::next(self) {
            Some(r) => Box::leak(Box::new(r)) as *mut _ as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }
}
