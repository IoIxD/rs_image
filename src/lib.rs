#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_unsafe)]

use std::{
    ffi::{c_char, CStr, CString},
    io::ErrorKind,
    os::raw::c_void,
};

mod iter;
mod thin;

use image::{EncodableLayout, GenericImageView, ImageError};
use iter::{make_raw_iterator, RawIterator};

/// Struct that contains function pointers that correspond to both Rust's Write trait and it's Seek trait.
/// The idea here is that you put the pointer for your custom own struct in the user_data field,
/// then pass your own functions which will then access that user data.
#[repr(C)]
pub struct RustWriter {
    pub user_data: *mut c_void,
    pub write_fn: extern "C" fn(ud: *mut c_void, buf: *const u8, buf_size: usize) -> usize,
    pub flush_fn: extern "C" fn(ud: *mut c_void) -> (),
    pub seek_fn: extern "C" fn(ud: *mut c_void, pos: SeekFrom) -> u64,
}

impl std::io::Write for RustWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok((self.write_fn)(
            self.user_data,
            buf.as_bytes().as_ptr(),
            buf.len(),
        ))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok((self.flush_fn)(self.user_data))
    }
}

impl std::io::Seek for RustWriter {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let raw_sp = match pos {
            std::io::SeekFrom::Start(s) => SeekFrom {
                ty: SeekType::SEEK_FROM_START,
                val: SeekUnion { start: s },
            },
            std::io::SeekFrom::End(e) => SeekFrom {
                ty: SeekType::SEEK_FROM_END,
                val: SeekUnion { end: e },
            },
            std::io::SeekFrom::Current(c) => SeekFrom {
                ty: SeekType::SEEK_FROM_CURRENT,
                val: SeekUnion { current: c },
            },
        };
        Ok((self.seek_fn)(self.user_data, raw_sp))
    }
}

/// Struct that contains function pointers that correspond to the image crate's ImageEncoder trait.
/// The idea here is that you put the pointer for your custom own struct in the user_data field,
/// then pass your own function to writeFn, which will then access that user data.
#[repr(C)]
pub struct ImageEncoder {
    pub user_data: *mut c_void,
    pub writeFn: extern "C" fn(
        ud: *mut c_void,
        buf: *const u8,
        size: usize,
        width: u32,
        height: u32,
        color_type: ExtendedColorType,
    ),
}

impl image::ImageEncoder for &mut ImageEncoder {
    fn write_image(
        self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: image::ExtendedColorType,
    ) -> image::ImageResult<()> {
        let c = color_type;
        let color_type: u16 = unsafe { std::mem::transmute(c) };
        (self.writeFn)(
            self.user_data,
            buf.as_ptr(),
            buf.len(),
            width,
            height,
            unsafe { std::mem::transmute(color_type as u32) },
        );
        Ok(())
    }
}

/// Enumeration of possible methods to seek within an I/O object.
/// ty represents whether you need to seek from the start, end, or current position
#[repr(C)]
pub struct SeekFrom {
    ty: SeekType,
    val: SeekUnion,
}

/// Union used for the SeekFrom struct
#[repr(C)]
pub union SeekUnion {
    /// Sets the offset to the provided number of bytes.
    start: u64,
    /// Sets the offset to the size of this object plus the specified number of bytes. It is possible to seek beyond the end of an object, but it’s an error to seek before byte 0.
    end: i64,
    /// Sets the offset to the current position plus the specified number of bytes. It is possible to seek beyond the end of an object, but it’s an error to seek before byte 0.
    current: i64,
}

#[repr(C)]
pub enum ErrorType {
    IMAGE_ERROR_NONE,
    IMAGE_ERROR_DECODING,
    IMAGE_ERROR_ENCODING,
    IMAGE_ERROR_PARAMETER_DIMENSION_MISMATCH,
    IMAGE_ERROR_PARAMETER_FAILED_ALREADY,
    IMAGE_ERROR_PARAMETER_MALFORMED,
    IMAGE_ERROR_PARAMETER_NO_MORE_DATA,
    IMAGE_ERROR_INSUFFICIENT_MEMORY,
    IMAGE_ERROR_LIMITS_UNSUPPORTED,
    IMAGE_ERROR_DIMENSION_ERROR,
    IMAGE_ERROR_UNSUPPORTED_COLOR,
    IMAGE_ERROR_UNSUPPORTED_FORMAT,
    IMAGE_ERROR_UNSUPPORTED_OTHER,
    IMAGE_ERROR_IO_NOT_FOUND,
    IMAGE_ERROR_IO_PERMISSION_DENIED,
    IMAGE_ERROR_IO_CONNECTION_REFUSED,
    IMAGE_ERROR_IO_CONNECTION_RESET,
    IMAGE_ERROR_IO_CONNECTION_ABORTED,
    IMAGE_ERROR_IO_NOT_CONNECTED,
    IMAGE_ERROR_IO_ADDR_IN_USE,
    IMAGE_ERROR_IO_ADDR_NOT_AVALIABLE,
    IMAGE_ERROR_IO_BROKEN_PIPE,
    IMAGE_ERROR_IO_ALREADY_EXISTS,
    IMAGE_ERROR_IO_WOULD_BLOCK,
    IMAGE_ERROR_IO_INVALID_INPUT,
    IMAGE_ERROR_IO_INVALID_DATA,
    IMAGE_ERROR_IO_TIMED_OUT,
    IMAGE_ERROR_IO_WRITE_ZERO,
    IMAGE_ERROR_IO_INTERRUPTED,
    IMAGE_ERROR_IO_UNSUPPORED,
    IMAGE_ERROR_IO_UNEXPECTED_EOF,
    IMAGE_ERROR_IO_OUT_OF_MEMORY,
    IMAGE_ERROR_IO_OTHER,
    IMAGE_ERROR_UNKNOWN,
}

fn get_image_error(er: ImageError) -> ErrorType {
    match er {
        image::ImageError::Decoding(_) => ErrorType::IMAGE_ERROR_DECODING,
        image::ImageError::Encoding(_) => ErrorType::IMAGE_ERROR_ENCODING,
        image::ImageError::Parameter(a) => match a.kind() {
            image::error::ParameterErrorKind::DimensionMismatch => {
                ErrorType::IMAGE_ERROR_PARAMETER_DIMENSION_MISMATCH
            }
            image::error::ParameterErrorKind::FailedAlready => {
                ErrorType::IMAGE_ERROR_PARAMETER_FAILED_ALREADY
            }
            image::error::ParameterErrorKind::Generic(_) => {
                ErrorType::IMAGE_ERROR_PARAMETER_MALFORMED
            }
            image::error::ParameterErrorKind::NoMoreData => {
                ErrorType::IMAGE_ERROR_PARAMETER_NO_MORE_DATA
            }
            _ => todo!(),
        },
        image::ImageError::Limits(a) => match a.kind() {
            image::error::LimitErrorKind::DimensionError => ErrorType::IMAGE_ERROR_DIMENSION_ERROR,
            image::error::LimitErrorKind::InsufficientMemory => {
                ErrorType::IMAGE_ERROR_INSUFFICIENT_MEMORY
            }
            image::error::LimitErrorKind::Unsupported { .. } => {
                ErrorType::IMAGE_ERROR_LIMITS_UNSUPPORTED
            }
            _ => todo!(),
        },
        image::ImageError::Unsupported(a) => match a.kind() {
            image::error::UnsupportedErrorKind::Color(_) => {
                ErrorType::IMAGE_ERROR_UNSUPPORTED_COLOR
            }
            image::error::UnsupportedErrorKind::Format(_) => {
                ErrorType::IMAGE_ERROR_UNSUPPORTED_FORMAT
            }
            image::error::UnsupportedErrorKind::GenericFeature(_) => {
                ErrorType::IMAGE_ERROR_UNSUPPORTED_OTHER
            }
            _ => todo!(),
        },
        image::ImageError::IoError(a) => match a.kind() {
            ErrorKind::NotFound => ErrorType::IMAGE_ERROR_IO_NOT_FOUND,
            ErrorKind::PermissionDenied => ErrorType::IMAGE_ERROR_IO_PERMISSION_DENIED,
            ErrorKind::ConnectionRefused => ErrorType::IMAGE_ERROR_IO_CONNECTION_REFUSED,
            ErrorKind::ConnectionReset => ErrorType::IMAGE_ERROR_IO_CONNECTION_RESET,
            ErrorKind::ConnectionAborted => ErrorType::IMAGE_ERROR_IO_CONNECTION_ABORTED,
            ErrorKind::NotConnected => ErrorType::IMAGE_ERROR_IO_NOT_CONNECTED,
            ErrorKind::AddrInUse => ErrorType::IMAGE_ERROR_IO_ADDR_IN_USE,
            ErrorKind::AddrNotAvailable => ErrorType::IMAGE_ERROR_IO_ADDR_NOT_AVALIABLE,
            ErrorKind::BrokenPipe => ErrorType::IMAGE_ERROR_IO_BROKEN_PIPE,
            ErrorKind::AlreadyExists => ErrorType::IMAGE_ERROR_IO_ALREADY_EXISTS,
            ErrorKind::WouldBlock => ErrorType::IMAGE_ERROR_IO_WOULD_BLOCK,
            ErrorKind::InvalidInput => ErrorType::IMAGE_ERROR_IO_INVALID_INPUT,
            ErrorKind::InvalidData => ErrorType::IMAGE_ERROR_IO_INVALID_DATA,
            ErrorKind::TimedOut => ErrorType::IMAGE_ERROR_IO_TIMED_OUT,
            ErrorKind::WriteZero => ErrorType::IMAGE_ERROR_IO_WRITE_ZERO,
            ErrorKind::Interrupted => ErrorType::IMAGE_ERROR_IO_INTERRUPTED,
            ErrorKind::Unsupported => ErrorType::IMAGE_ERROR_IO_UNSUPPORED,
            ErrorKind::UnexpectedEof => ErrorType::IMAGE_ERROR_IO_UNEXPECTED_EOF,
            ErrorKind::OutOfMemory => ErrorType::IMAGE_ERROR_IO_OUT_OF_MEMORY,
            ErrorKind::Other => ErrorType::IMAGE_ERROR_IO_OTHER,
            _ => ErrorType::IMAGE_ERROR_UNKNOWN, // This means Rust added more errors then the last time this was updated.
        },
    }
}

#[repr(C)]
pub enum FilterType {
    FILTER_TYPE_NEAREST,
    FILTER_TYPE_TRIANGLE,
    FILTER_TYPE_CATMULL_ROM,
    FILTER_TYPE_GAUSSIAN,
    FILTER_TYPE_LANCZOS3,
}

#[repr(C)]
pub enum ImageFormat {
    IMAGE_FORMAT_PNG,
    IMAGE_FORMAT_JPEG,
    IMAGE_FORMAT_GIF,
    IMAGE_FORMAT_WEBP,
    IMAGE_FORMAT_PNM,
    IMAGE_FORMAT_TIFF,
    IMAGE_FORMAT_TGA,
    IMAGE_FORMAT_DDS,
    IMAGE_FORMAT_BMP,
    IMAGE_FORMAT_ICO,
    IMAGE_FORMAT_HDR,
    IMAGE_FORMAT_OPENEXR,
    IMAGE_FORMAT_FARBFELD,
    IMAGE_FORMAT_AVIF,
    IMAGE_FORMAT_QOI,
}
#[repr(C)]
pub enum ColorType {
    COLOR_TYPE_L8,
    COLOR_TYPE_LA8,
    COLOR_TYPE_RGB8,
    COLOR_TYPE_RGBA8,
    COLOR_TYPE_L16,
    COLOR_TYPE_LA16,
    COLOR_TYPE_RGB16,
    COLOR_TYPE_RGBA16,
    COLOR_TYPE_RGB32F,
    COLOR_TYPE_RGBA32F,
}

#[repr(C)]
pub enum ExtendedColorType {
    EXTENDED_COLOR_TYPE_A8,
    EXTENDED_COLOR_TYPE_L1,
    EXTENDED_COLOR_TYPE_LA1,
    EXTENDED_COLOR_TYPE_RGB1,
    EXTENDED_COLOR_TYPE_RGBA1,
    EXTENDED_COLOR_TYPE_L2,
    EXTENDED_COLOR_TYPE_LA2,
    EXTENDED_COLOR_TYPE_RGB2,
    EXTENDED_COLOR_TYPE_RGBA2,
    EXTENDED_COLOR_TYPE_L4,
    EXTENDED_COLOR_TYPE_LA4,
    EXTENDED_COLOR_TYPE_RGB4,
    EXTENDED_COLOR_TYPE_RGBA4,
    EXTENDED_COLOR_TYPE_L8,
    EXTENDED_COLOR_TYPE_LA8,
    EXTENDED_COLOR_TYPE_RGB8,
    EXTENDED_COLOR_TYPE_RGBA8,
    EXTENDED_COLOR_TYPE_L16,
    EXTENDED_COLOR_TYPE_LA16,
    EXTENDED_COLOR_TYPE_RGB16,
    EXTENDED_COLOR_TYPE_RGBA16,
    EXTENDED_COLOR_TYPE_BGR8,
    EXTENDED_COLOR_TYPE_BGRA8,
    EXTENDED_COLOR_TYPE_RGB32F,
    EXTENDED_COLOR_TYPE_RGBA32F,
    EXTENDED_COLOR_TYPE_CMYK8,
}

/// Type used for the SeekFrom struct
#[repr(C)]
pub enum SeekType {
    SEEK_FROM_START,
    SEEK_FROM_END,
    SEEK_FROM_CURRENT,
}

#[repr(C)]
pub struct DynamicImage {
    inner: *mut c_void,
}

impl DynamicImage {
    fn to_real(&self) -> &image::DynamicImage {
        assert!(!self.inner.is_null());
        assert!(self.inner.is_aligned());
        let ptr = self.inner;
        if ptr.is_null() {
            panic!("FATAL: Tried to call method on DynamicImage, which is null");
        }
        return unsafe { (ptr as *const image::DynamicImage).as_ref().unwrap() };
    }
    fn to_real_mut(&self) -> &'static mut image::DynamicImage {
        assert!(!self.inner.is_null());
        assert!(self.inner.is_aligned());
        let ptr = self.inner;
        return unsafe { (ptr as *mut image::DynamicImage).as_mut().unwrap() };
    }
}

macro_rules! unravel {
    ($self:tt) => {{
        assert!(!$self.is_null());
        assert!($self.is_aligned());
        unsafe { $self.as_ref() }.unwrap().to_real()
    }};
}

macro_rules! unravel_mut {
    ($self:tt) => {{
        assert!(!$self.is_null());
        assert!($self.is_aligned());
        unsafe { $self.as_mut() }.unwrap().to_real_mut()
    }};
}

macro_rules! ravel {
    ($f:block) => {{
        Box::leak(Box::new(DynamicImage {
            inner: Box::leak(Box::new($f)) as *mut image::DynamicImage as *mut c_void,
        }))
    }};
}

#[repr(C)]
pub struct LoadFromMemoryResult {
    pub res: *mut DynamicImage,
    pub err: ErrorType,
}

#[no_mangle]
pub extern "C" fn dynamic_image_load_from_memory(
    bytes: *mut u8,
    size: usize,
) -> LoadFromMemoryResult {
    assert!(!bytes.is_null());
    assert!(bytes.is_aligned());
    assert!(size < isize::MAX as usize);
    match image::load_from_memory(unsafe { std::slice::from_raw_parts_mut(bytes, size) }) {
        Ok(a) => LoadFromMemoryResult {
            res: ravel!({ a }),
            err: ErrorType::IMAGE_ERROR_NONE,
        },
        Err(er) => LoadFromMemoryResult {
            res: std::ptr::null_mut(),
            err: get_image_error(er),
        },
    }
}

//#[no_mangle]
//pub extern "C" fn dynamic_image_from_decoder() {}

#[no_mangle]
pub extern "C" fn dynamic_image_adjust_contrast(
    this: *mut DynamicImage,
    c: f32,
) -> *mut DynamicImage {
    ravel!({ unravel_mut!(this).adjust_contrast(c) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_as_bytes(this: *mut DynamicImage, count: *mut usize) -> *mut u8 {
    let b = unravel_mut!(this).as_bytes();
    unsafe { *count = b.len() };
    return Box::leak(Box::new(unsafe { *b.as_ptr() }));
}

#[no_mangle]
pub extern "C" fn dynamic_image_blur(this: *mut DynamicImage, sigma: f32) -> *mut DynamicImage {
    ravel!({ unravel_mut!(this).blur(sigma) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_brighten(this: *mut DynamicImage, value: i32) -> *mut DynamicImage {
    ravel!({ unravel_mut!(this).brighten(value) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_invert(this: *mut DynamicImage) {
    unravel_mut!(this).invert();
}
#[no_mangle]
pub extern "C" fn dynamic_image_color(this: *mut DynamicImage) -> ColorType {
    let col = unravel_mut!(this).color() as u32;
    unsafe { std::mem::transmute(col) }
}
#[no_mangle]
pub extern "C" fn dynamic_image_crop(
    this: *mut DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> *mut DynamicImage {
    ravel!({ unravel_mut!(this).crop(x, y, width, height) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_crop_imm(
    this: *mut DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> *mut DynamicImage {
    ravel!({ unravel_mut!(this).crop_imm(x, y, width, height) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_filter3x3(
    this: *mut DynamicImage,
    kernel: *mut f32,
    size: usize,
) -> *mut DynamicImage {
    assert!(!kernel.is_null());
    assert!(kernel.is_aligned());
    assert!(size < isize::MAX as usize);
    ravel!({
        unravel_mut!(this).filter3x3(unsafe { std::slice::from_raw_parts_mut(kernel, size) })
    })
}
#[no_mangle]
pub extern "C" fn dynamic_image_fliph(this: *mut DynamicImage) -> *mut DynamicImage {
    ravel!({ unravel_mut!(this).fliph() })
}
#[no_mangle]
pub extern "C" fn dynamic_image_flipv(this: *mut DynamicImage) -> *mut DynamicImage {
    ravel!({ unravel_mut!(this).flipv() })
}

#[no_mangle]
pub extern "C" fn dynamic_image_grayscale(this: *mut DynamicImage) -> *mut DynamicImage {
    ravel!({ unravel_mut!(this).grayscale() })
}
#[no_mangle]
pub extern "C" fn dynamic_image_unsharpen(
    this: *mut DynamicImage,
    sigma: f32,
    threshold: i32,
) -> *mut DynamicImage {
    ravel!({ unravel_mut!(this).unsharpen(sigma, threshold) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_width(this: *mut DynamicImage) -> u32 {
    unravel_mut!(this).width()
}
#[no_mangle]
pub extern "C" fn dynamic_image_height(this: *mut DynamicImage) -> u32 {
    unravel_mut!(this).height()
}
#[no_mangle]
pub extern "C" fn dynamic_image_huerotate(
    this: *mut DynamicImage,
    value: i32,
) -> *mut DynamicImage {
    ravel!({ unravel_mut!(this).huerotate(value) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_into_bytes(
    this: *const DynamicImage,
    size: *mut usize,
) -> *mut *const u8 {
    let dynamic_image = &unravel!(this);
    let b = dynamic_image.as_bytes();
    unsafe { *size = b.len() };
    return Box::leak(Box::new(b.as_ptr()));
}

#[no_mangle]
pub extern "C" fn dynamic_image_resize(
    this: *mut DynamicImage,
    nwidth: u32,
    nheight: u32,
    filter: FilterType,
) -> *mut DynamicImage {
    let filter = unsafe { std::mem::transmute(filter as u8) };
    ravel!({ unravel!(this).resize(nwidth, nheight, filter) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_resize_exact(
    this: *mut DynamicImage,
    nwidth: u32,
    nheight: u32,
    filter: FilterType,
) -> *mut DynamicImage {
    let filter = unsafe { std::mem::transmute(filter as u8) };
    ravel!({ unravel!(this).resize_exact(nwidth, nheight, filter) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_resize_to_fill(
    this: *mut DynamicImage,
    nwidth: u32,
    nheight: u32,
    filter: FilterType,
) -> *mut DynamicImage {
    let filter = unsafe { std::mem::transmute(filter as u8) };
    ravel!({ unravel!(this).resize_to_fill(nwidth, nheight, filter) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_rotate180(this: *mut DynamicImage) -> *mut DynamicImage {
    ravel!({ unravel!(this).rotate180() })
}
#[no_mangle]
pub extern "C" fn dynamic_image_rotate270(this: *mut DynamicImage) -> *mut DynamicImage {
    ravel!({ unravel!(this).rotate270() })
}
#[no_mangle]
pub extern "C" fn dynamic_image_rotate90(this: *mut DynamicImage) -> *mut DynamicImage {
    ravel!({ unravel!(this).rotate90() })
}
#[no_mangle]
pub extern "C" fn dynamic_image_save(
    this: *mut DynamicImage,
    path: *const c_char,
) -> *const c_char {
    match unravel!(this).save(unsafe { CStr::from_ptr(path) }.to_str().unwrap()) {
        Ok(_) => std::ptr::null(),
        Err(er) => unsafe {
            Box::leak(Box::new(
                CString::new(format!("{:?}", er).as_str()).unwrap(),
            ))
            .as_ptr()
        },
    }
}
#[no_mangle]
pub extern "C" fn dynamic_image_save_with_format(
    this: *mut DynamicImage,
    path: *const c_char,
    format: ImageFormat,
) -> ErrorType {
    let format = unsafe { std::mem::transmute(format as u8) };
    match unravel!(this).save_with_format(unsafe { CStr::from_ptr(path) }.to_str().unwrap(), format)
    {
        Ok(_) => ErrorType::IMAGE_ERROR_NONE,
        Err(er) => get_image_error(er),
    }
}
#[no_mangle]
pub extern "C" fn dynamic_image_thumbnail(
    this: *mut DynamicImage,
    nwidth: u32,
    nheight: u32,
) -> *mut DynamicImage {
    ravel!({ unravel!(this).thumbnail(nwidth, nheight) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_thumbnail_exact(
    this: *mut DynamicImage,
    nwidth: u32,
    nheight: u32,
) -> *mut DynamicImage {
    ravel!({ unravel!(this).thumbnail_exact(nwidth, nheight) })
}

#[no_mangle]
pub extern "C" fn dynamic_image_into_luma16(this: *mut DynamicImage) -> *mut DynamicImage {
    let th = unravel!(this).to_owned();
    ravel!({ image::DynamicImage::from(th.into_luma16()) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_into_luma8(this: *mut DynamicImage) -> *mut DynamicImage {
    let th = unravel!(this).to_owned();
    ravel!({ image::DynamicImage::from(th.into_luma8()) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_into_luma_alpha16(this: *mut DynamicImage) -> *mut DynamicImage {
    let th = unravel!(this).to_owned();
    ravel!({ image::DynamicImage::from(th.into_luma_alpha16()) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_into_luma_alpha8(this: *mut DynamicImage) -> *mut DynamicImage {
    let th = unravel!(this).to_owned();
    ravel!({ image::DynamicImage::from(th.into_luma_alpha8()) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_into_rgb16(this: *mut DynamicImage) -> *mut DynamicImage {
    let th = unravel!(this).to_owned();
    ravel!({ image::DynamicImage::from(th.into_rgb16()) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_into_rgb32f(this: *mut DynamicImage) -> *mut DynamicImage {
    let th = unravel!(this).to_owned();
    ravel!({ image::DynamicImage::from(th.into_rgb32f()) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_into_rgb8(this: *mut DynamicImage) -> *mut DynamicImage {
    let th = unravel!(this).to_owned();
    ravel!({ image::DynamicImage::from(th.into_rgb8()) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_into_rgba16(this: *mut DynamicImage) -> *mut DynamicImage {
    let th = unravel!(this).to_owned();
    ravel!({ image::DynamicImage::from(th.into_rgba16()) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_into_rgba32f(this: *mut DynamicImage) -> *mut DynamicImage {
    let th = unravel!(this).to_owned();
    ravel!({ image::DynamicImage::from(th.into_rgba32f()) })
}
#[no_mangle]
pub extern "C" fn dynamic_image_into_rgba8(this: *mut DynamicImage) -> *mut DynamicImage {
    let th = unravel!(this).to_owned();
    ravel!({ image::DynamicImage::from(th.into_rgba8()) })
}

#[no_mangle]
pub extern "C" fn dynamic_image_write_to(
    this: *mut DynamicImage,
    w: *mut RustWriter,
    format: ImageFormat,
) -> ErrorType {
    let format = unsafe { std::mem::transmute(format as u8) };
    match unravel!(this).write_to(unsafe { &mut w.as_mut().unwrap() }, format) {
        Ok(_) => ErrorType::IMAGE_ERROR_NONE,
        Err(er) => get_image_error(er),
    }
}
#[no_mangle]
pub extern "C" fn dynamic_image_write_with_encoder(
    this: *mut DynamicImage,
    encoder: *mut ImageEncoder,
) -> ErrorType {
    match unravel!(this).write_with_encoder(unsafe { encoder.as_mut().unwrap() }) {
        Ok(_) => ErrorType::IMAGE_ERROR_NONE,
        Err(er) => get_image_error(er),
    }
}

#[repr(C)]
pub struct Dimensions {
    width: u32,
    height: u32,
}

#[no_mangle]
pub extern "C" fn dynamic_image_dimensions(this: *mut DynamicImage) -> Dimensions {
    unsafe { std::mem::transmute(unravel!(this).dimensions()) }
}

#[repr(C)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[no_mangle]
pub extern "C" fn dynamic_image_get_pixel(this: *mut DynamicImage, x: u32, y: u32) -> Rgba {
    let dynamic_image = unravel!(this);
    let c = dynamic_image.get_pixel(x, y);
    let col = c.0;
    let e = Rgba {
        r: col[0],
        g: col[1],
        b: col[2],
        a: col[3],
    };
    e
}

#[no_mangle]
pub extern "C" fn dynamic_image_in_bounds(this: *mut DynamicImage, x: u32, y: u32) -> bool {
    unravel!(this).in_bounds(x, y)
}

#[no_mangle]
pub extern "C" fn dynamic_image_pixels(this: *mut DynamicImage) -> RawIterator {
    make_raw_iterator(unravel!(this).pixels())
}

#[no_mangle]
pub extern "C" fn dynamic_image_free(this: *mut DynamicImage) {
    if !this.is_null() {
        let og = unsafe { Box::from_raw(this) };
        if og.inner != std::ptr::null_mut() {
            std::mem::drop(unsafe { Box::from_raw(og.inner) });
        }
        std::mem::drop(og);
        // otherwise, do nothing. the value doesn't exist.
    }
}

#[repr(C)]
pub struct PixelResult {
    pub x: u32,
    pub y: u32,
    pub color: Rgba,
}

#[no_mangle]
pub extern "C" fn ____() -> PixelResult {
    unimplemented!()
}
