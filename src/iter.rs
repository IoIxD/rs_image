use std::ffi::c_void;

#[repr(C)]
pub enum Ordering {
    ORDERING_LESS,
    ORDERING_EQUAL,
    ORDERING_GREATER,
}

impl Ordering {
    fn from_std(real: std::cmp::Ordering) -> Self {
        match real {
            std::cmp::Ordering::Less => Ordering::ORDERING_LESS,
            std::cmp::Ordering::Equal => Ordering::ORDERING_EQUAL,
            std::cmp::Ordering::Greater => Ordering::ORDERING_GREATER,
        }
    }
    /*fn to_std(&self) -> std::cmp::Ordering {
        match self {
            Ordering::ORDERING_LESS => std::cmp::Ordering::Less,
            Ordering::ORDERING_EQUAL => std::cmp::Ordering::Equal,
            Ordering::ORDERING_GREATER => std::cmp::Ordering::Greater,
        }
    }*/
}

#[repr(C)]
pub struct SizeHint {
    lhs: usize,
    rhs: *mut usize,
}

/*#[repr(C)]
pub struct PartitionResult {
    lhs: *mut c_void,
    rhs: *mut c_void,
}*/

/// A wrapper for the Rust iterator to C. You generally get this from one of the provided library functions.
///
/// `__s` is expected to a pointer to something that implements Rust's std::iter::Iterator. You should not try and instantiate this yourself unless you have an object from Rust code.
///
/// To make use of this, you should use the appropriate `iter_...` function.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RawIterator {
    // The inner iterator pointer
    __s: *mut BoxedThinIterator<'static>,
    // If we do a collection or sum, the length of the resulting array is stored here
    __size: usize,
}

use crate::thin::{BoxedThinIterator, ThinIterator};

pub extern "C" fn make_raw_iterator(mut iter: impl ThinIterator + Sized + 'static) -> RawIterator {
    return RawIterator {
        __s: Box::leak(Box::new(BoxedThinIterator::new(iter))) as *mut BoxedThinIterator,
        __size: 0,
    };
}

impl Iterator for RawIterator {
    type Item = *mut c_void;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { ThinIterator::next(self.__s.as_mut().unwrap()).as_mut() } {
            Some(a) => Some(a),
            None => None,
        }
    }
}

fn to_inner<'a>(s: *mut RawIterator) -> &'a RawIterator {
    unsafe { s.as_ref().expect("iterator is null!") }
}

fn to_inner_mut<'a>(s: *mut RawIterator) -> &'a mut RawIterator {
    unsafe { s.as_mut().expect("iterator is null!") }
}
#[no_mangle]
extern "C" fn iter_next(s: *mut RawIterator) -> *mut c_void {
    match to_inner_mut(s).next() {
        Some(a) => a,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
extern "C" fn iter_size_hint(s: *mut RawIterator) -> SizeHint {
    let r = to_inner(s).size_hint();
    SizeHint {
        lhs: r.0,
        rhs: match r.1 {
            Some(a) => Box::leak(Box::new(a)),
            None => std::ptr::null_mut(),
        },
    }
}
#[no_mangle]
extern "C" fn iter_count(s: *mut RawIterator) -> usize {
    to_inner(s).count()
}
#[no_mangle]
extern "C" fn iter_last(s: *mut RawIterator) -> *mut c_void {
    match to_inner(s).last() {
        Some(a) => Box::leak(Box::new(a)) as *mut _ as *mut c_void,
        None => todo!(),
    }
}
#[no_mangle]
extern "C" fn iter_nth(s: *mut RawIterator, n: usize) -> *mut c_void {
    match to_inner_mut(s).nth(n) {
        Some(a) => Box::leak(Box::new(a)) as *mut _ as *mut c_void,
        None => todo!(),
    }
}
#[no_mangle]
extern "C" fn iter_step_by(s: *mut RawIterator, step: usize) -> RawIterator {
    make_raw_iterator(unsafe { s.as_ref().unwrap().step_by(step) })
}
#[no_mangle]
extern "C" fn iter_chain(s: *mut RawIterator, other: RawIterator) -> RawIterator {
    make_raw_iterator(unsafe { s.as_ref().unwrap().chain(other) })
}
#[no_mangle]
extern "C" fn iter_zip(s: *mut RawIterator, other: RawIterator) -> RawIterator {
    make_raw_iterator(to_inner(s).zip(other))
}

#[no_mangle]
extern "C" fn iter_map(s: *mut RawIterator, f: extern "C" fn(*mut c_void)) -> RawIterator {
    make_raw_iterator(to_inner(s).map(move |a| f(a)))
}

#[no_mangle]
extern "C" fn iter_for_each(s: *mut RawIterator, f: extern "C" fn(*mut c_void)) {
    to_inner(s).for_each(move |a| f(a));
}

#[no_mangle]
extern "C" fn iter_filter(
    s: *mut RawIterator,
    predicate: extern "C" fn(*mut c_void) -> bool,
) -> RawIterator {
    make_raw_iterator(to_inner(s).filter(move |a| predicate(*a)))
}
#[no_mangle]
extern "C" fn iter_filter_map(
    s: *mut RawIterator,
    f: extern "C" fn(*mut c_void) -> *mut RawIterator,
) -> RawIterator {
    make_raw_iterator(
        to_inner(s).filter_map(move |a| match unsafe { f(a).as_ref() } {
            Some(mut a) => Some(*a),
            None => None,
        }),
    )
}
#[no_mangle]
extern "C" fn iter_enumerate(s: *mut RawIterator) -> RawIterator {
    make_raw_iterator(to_inner(s).enumerate())
}
#[no_mangle]
extern "C" fn iter_peekable(s: *mut RawIterator) -> RawIterator {
    make_raw_iterator(to_inner(s).peekable())
}
#[no_mangle]
extern "C" fn iter_skip_while(
    s: *mut RawIterator,
    predicate: extern "C" fn(*mut c_void) -> bool,
) -> RawIterator {
    make_raw_iterator(to_inner(s).skip_while(move |a| predicate(*a)))
}
#[no_mangle]
extern "C" fn iter_take_while(
    s: *mut RawIterator,
    predicate: extern "C" fn(*mut c_void) -> bool,
) -> RawIterator {
    make_raw_iterator(to_inner(s).take_while(move |a| predicate(*a)))
}
#[no_mangle]
extern "C" fn iter_map_while(
    s: *mut RawIterator,
    predicate: extern "C" fn(*mut c_void) -> *mut RawIterator,
) -> RawIterator {
    make_raw_iterator(to_inner(s).map_while(move |a: *mut c_void| {
        match unsafe { predicate(a).as_ref() } {
            Some(a) => Some(*a),
            None => None,
        }
    }))
}
#[no_mangle]
extern "C" fn iter_skip(s: *mut RawIterator, n: usize) -> RawIterator {
    make_raw_iterator(to_inner(s).skip(n))
}
#[no_mangle]
extern "C" fn iter_take(s: *mut RawIterator, n: usize) -> RawIterator {
    make_raw_iterator(to_inner(s).take(n))
}
#[no_mangle]
extern "C" fn iter_scan(
    s: *mut RawIterator,
    initial_state: *mut c_void,
    f: extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
) -> RawIterator {
    make_raw_iterator(to_inner(s).scan(initial_state, move |a, b| unsafe { f(*a, b).as_ref() }))
}
#[no_mangle]
extern "C" fn iter_flat_map(
    s: *mut RawIterator,
    f: extern "C" fn(*mut c_void) -> RawIterator,
) -> RawIterator {
    make_raw_iterator(to_inner(s).flat_map(move |a| f(a)))
}
#[no_mangle]
extern "C" fn iter_fuse(s: *mut RawIterator) -> RawIterator {
    make_raw_iterator(to_inner(s).fuse())
}
#[no_mangle]
extern "C" fn iter_inspect(s: *mut RawIterator, f: extern "C" fn(*mut c_void)) -> RawIterator {
    make_raw_iterator(to_inner(s).inspect(move |a| f(*a)))
}
#[no_mangle]
extern "C" fn iter_by_ref(s: *mut RawIterator) -> RawIterator {
    make_raw_iterator(to_inner_mut(s).by_ref())
}
#[no_mangle]
extern "C" fn iter_collect(s: *mut RawIterator, size: &mut usize) -> *mut *mut c_void {
    let mut v: Vec<*mut c_void> = to_inner_mut(s).collect();
    *size = v.len();
    Box::leak(Box::new(v.as_mut_slice())).as_mut_ptr()
}

#[no_mangle]
extern "C" fn iter_fold(
    s: *mut RawIterator,
    init: *mut c_void,
    f: extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
) -> *mut c_void {
    to_inner(s).fold(init, |a, b| f(a, b))
}
#[no_mangle]
extern "C" fn iter_reduce(
    s: *mut RawIterator,
    f: extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
) -> *mut c_void {
    match to_inner(s).reduce(|s, a| f(s, a)) {
        Some(a) => a,
        None => std::ptr::null_mut(),
    }
}
#[no_mangle]
extern "C" fn iter_all(s: *mut RawIterator, f: extern "C" fn(*mut c_void) -> bool) -> bool {
    to_inner_mut(s).all(|a| f(a))
}
#[no_mangle]
extern "C" fn iter_any(s: *mut RawIterator, f: extern "C" fn(*mut c_void) -> bool) -> bool {
    to_inner_mut(s).any(|a| f(a))
}
#[no_mangle]
extern "C" fn iter_find(
    s: *mut RawIterator,
    predicate: extern "C" fn(*mut c_void) -> bool,
) -> *mut c_void {
    match to_inner_mut(s).find(|p| predicate(*p)) {
        Some(a) => a,
        None => std::ptr::null_mut(),
    }
}
#[no_mangle]
extern "C" fn iter_find_map(
    s: *mut RawIterator,
    f: extern "C" fn(*mut c_void) -> *mut c_void,
) -> *mut c_void {
    match to_inner_mut(s).find_map(|a| unsafe { f(a).as_mut() }) {
        Some(a) => a as *mut c_void,
        None => std::ptr::null_mut(),
    }
}
#[no_mangle]
extern "C" fn iter_position(
    s: *mut RawIterator,
    predicate: extern "C" fn(*mut c_void) -> bool,
) -> *mut usize {
    match to_inner_mut(s).position(|a| predicate(a)) {
        Some(a) => Box::leak(Box::new(a)),
        None => std::ptr::null_mut(),
    }
}
#[no_mangle]
extern "C" fn iter_max(s: *mut RawIterator) -> *mut c_void {
    match to_inner(s).max() {
        Some(a) => a,
        None => std::ptr::null_mut(),
    }
}
#[no_mangle]
extern "C" fn iter_min(s: *mut RawIterator) -> *mut c_void {
    match to_inner(s).min() {
        Some(a) => a,
        None => std::ptr::null_mut(),
    }
}
/*#[no_mangle]
extern "C" fn iter_max_by_key(
    s: *mut RawIterator,
    f: extern "C" fn(*mut c_void) -> *mut c_void,
) -> *mut c_void {
    match to_inner(s).max_by_key(|compare| f(*compare)) {
        Some(a) => a,
        None => std::ptr::null_mut(),
    }
}
#[no_mangle]
extern "C" fn iter_max_by(
    s: *mut RawIterator,
    compare: extern "C" fn(*mut c_void, *mut c_void) -> Ordering,
) -> *mut c_void {
    match to_inner(s).max_by(|a, b| compare(*a, *b).to_std()) {
        Some(a) => a,
        None => std::ptr::null_mut(),
    }
}
#[no_mangle]
extern "C" fn iter_min_by_key(
    s: *mut RawIterator,
    f: extern "C" fn(*mut c_void) -> *mut c_void,
) -> *mut c_void {
    match to_inner(s).min_by_key(|compare| f(*compare)) {
        Some(a) => a,
        None => std::ptr::null_mut(),
    }
}
#[no_mangle]
extern "C" fn iter_min_by(
    s: *mut RawIterator,
    compare: extern "C" fn(*mut c_void, *mut c_void) -> Ordering,
) -> *mut c_void {
    match to_inner(s).min_by(|a, b| compare(*a, *b).to_std()) {
        Some(a) => a,
        None => std::ptr::null_mut(),
    }
}*/

/*#[no_mangle]
extern "C" fn iter_sum(s: *mut RawIterator, size: &mut usize) -> *mut *mut c_void {

}*/

#[no_mangle]
extern "C" fn iter_cmp(s: *mut RawIterator, other: RawIterator) -> Ordering {
    Ordering::from_std(to_inner(s).cmp(other))
}
#[no_mangle]
extern "C" fn iter_partial_cmp(s: *mut RawIterator, other: RawIterator) -> *mut Ordering {
    match to_inner(s).partial_cmp(other) {
        Some(a) => Box::leak(Box::new(Ordering::from_std(a))),
        None => std::ptr::null_mut(),
    }
}
#[no_mangle]
extern "C" fn iter_eq(s: *mut RawIterator, other: RawIterator) -> bool {
    to_inner(s).eq(other)
}
#[no_mangle]
extern "C" fn iter_ne(s: *mut RawIterator, other: RawIterator) -> bool {
    to_inner(s).ne(other)
}
#[no_mangle]
extern "C" fn iter_lt(s: *mut RawIterator, other: RawIterator) -> bool {
    to_inner(s).lt(other)
}
#[no_mangle]
extern "C" fn iter_le(s: *mut RawIterator, other: RawIterator) -> bool {
    to_inner(s).le(other)
}
#[no_mangle]
extern "C" fn iter_gt(s: *mut RawIterator, other: RawIterator) -> bool {
    to_inner(s).gt(other)
}
#[no_mangle]
extern "C" fn iter_ge(s: *mut RawIterator, other: RawIterator) -> bool {
    to_inner(s).ge(other)
}
