#ifndef __INTERNAL_IMAGE_LOAD_H
#define __INTERNAL_IMAGE_LOAD_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum ColorType {
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
} ColorType;

typedef enum ErrorType {
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
} ErrorType;

typedef enum ExtendedColorType {
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
} ExtendedColorType;

typedef enum FilterType {
  FILTER_TYPE_NEAREST,
  FILTER_TYPE_TRIANGLE,
  FILTER_TYPE_CATMULL_ROM,
  FILTER_TYPE_GAUSSIAN,
  FILTER_TYPE_LANCZOS3,
} FilterType;

typedef enum ImageFormat {
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
} ImageFormat;

typedef enum Ordering {
  ORDERING_LESS,
  ORDERING_EQUAL,
  ORDERING_GREATER,
} Ordering;

/**
 * Type used for the SeekFrom struct
 */
typedef enum SeekType {
  SEEK_FROM_START,
  SEEK_FROM_END,
  SEEK_FROM_CURRENT,
} SeekType;

typedef struct DynamicImage {
  void *inner;
} DynamicImage;

typedef struct LoadFromMemoryResult {
  struct DynamicImage *res;
  enum ErrorType err;
} LoadFromMemoryResult;

/**
 * Union used for the SeekFrom struct
 */
typedef union SeekUnion {
  /**
   * Sets the offset to the provided number of bytes.
   */
  uint64_t start;
  /**
   * Sets the offset to the size of this object plus the specified number of bytes. It is possible to seek beyond the end of an object, but it’s an error to seek before byte 0.
   */
  int64_t end;
  /**
   * Sets the offset to the current position plus the specified number of bytes. It is possible to seek beyond the end of an object, but it’s an error to seek before byte 0.
   */
  int64_t current;
} SeekUnion;

/**
 * Enumeration of possible methods to seek within an I/O object.
 * ty represents whether you need to seek from the start, end, or current position
 */
typedef struct SeekFrom {
  enum SeekType ty;
  union SeekUnion val;
} SeekFrom;

/**
 * Struct that contains function pointers that correspond to both Rust's Write trait and it's Seek trait.
 * The idea here is that you put the pointer for your custom own struct in the user_data field,
 * then pass your own functions which will then access that user data.
 */
typedef struct RustWriter {
  void *user_data;
  uintptr_t (*write_fn)(void *ud, const uint8_t *buf, uintptr_t buf_size);
  void (*flush_fn)(void *ud);
  uint64_t (*seek_fn)(void *ud, struct SeekFrom pos);
} RustWriter;

/**
 * Struct that contains function pointers that correspond to the image crate's ImageEncoder trait.
 * The idea here is that you put the pointer for your custom own struct in the user_data field,
 * then pass your own function to writeFn, which will then access that user data.
 */
typedef struct ImageEncoder {
  void *user_data;
  void (*writeFn)(void *ud,
                  const uint8_t *buf,
                  uintptr_t size,
                  uint32_t width,
                  uint32_t height,
                  enum ExtendedColorType color_type);
} ImageEncoder;

typedef struct Dimensions {
  uint32_t width;
  uint32_t height;
} Dimensions;

typedef struct Rgba {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
} Rgba;

typedef struct ThinIteratorVtable {
  void *(*next)(void*);
  void (*drop)(void*);
} ThinIteratorVtable;

typedef struct ThinIteratorVtable *BoxedThinIterator;

/**
 * A wrapper for the Rust iterator to C. You generally get this from one of the provided library functions.
 *
 * `__s` is expected to a pointer to something that implements Rust's std::iter::Iterator. You should not try and instantiate this yourself unless you have an object from Rust code.
 *
 * To make use of this, you should use the appropriate `iter_...` function.
 */
typedef struct RawIterator {
  BoxedThinIterator *__s;
  uintptr_t __size;
} RawIterator;

typedef struct PixelResult {
  uint32_t x;
  uint32_t y;
  struct Rgba color;
} PixelResult;

typedef struct SizeHint {
  uintptr_t lhs;
  uintptr_t *rhs;
} SizeHint;

struct LoadFromMemoryResult dynamic_image_load_from_memory(uint8_t *bytes, uintptr_t size);

struct DynamicImage *dynamic_image_adjust_contrast(struct DynamicImage *this_, float c);

uint8_t *dynamic_image_as_bytes(struct DynamicImage *this_, uintptr_t *count);

struct DynamicImage *dynamic_image_blur(struct DynamicImage *this_, float sigma);

struct DynamicImage *dynamic_image_brighten(struct DynamicImage *this_, int32_t value);

void dynamic_image_invert(struct DynamicImage *this_);

enum ColorType dynamic_image_color(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_crop(struct DynamicImage *this_,
                                        uint32_t x,
                                        uint32_t y,
                                        uint32_t width,
                                        uint32_t height);

struct DynamicImage *dynamic_image_crop_imm(struct DynamicImage *this_,
                                            uint32_t x,
                                            uint32_t y,
                                            uint32_t width,
                                            uint32_t height);

struct DynamicImage *dynamic_image_filter3x3(struct DynamicImage *this_,
                                             float *kernel,
                                             uintptr_t size);

struct DynamicImage *dynamic_image_fliph(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_flipv(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_grayscale(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_unsharpen(struct DynamicImage *this_,
                                             float sigma,
                                             int32_t threshold);

uint32_t dynamic_image_width(struct DynamicImage *this_);

uint32_t dynamic_image_height(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_huerotate(struct DynamicImage *this_, int32_t value);

const uint8_t **dynamic_image_into_bytes(const struct DynamicImage *this_, uintptr_t *size);

struct DynamicImage *dynamic_image_resize(struct DynamicImage *this_,
                                          uint32_t nwidth,
                                          uint32_t nheight,
                                          enum FilterType filter);

struct DynamicImage *dynamic_image_resize_exact(struct DynamicImage *this_,
                                                uint32_t nwidth,
                                                uint32_t nheight,
                                                enum FilterType filter);

struct DynamicImage *dynamic_image_resize_to_fill(struct DynamicImage *this_,
                                                  uint32_t nwidth,
                                                  uint32_t nheight,
                                                  enum FilterType filter);

struct DynamicImage *dynamic_image_rotate180(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_rotate270(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_rotate90(struct DynamicImage *this_);

const char *dynamic_image_save(struct DynamicImage *this_, const char *path);

enum ErrorType dynamic_image_save_with_format(struct DynamicImage *this_,
                                              const char *path,
                                              enum ImageFormat format);

struct DynamicImage *dynamic_image_thumbnail(struct DynamicImage *this_,
                                             uint32_t nwidth,
                                             uint32_t nheight);

struct DynamicImage *dynamic_image_thumbnail_exact(struct DynamicImage *this_,
                                                   uint32_t nwidth,
                                                   uint32_t nheight);

struct DynamicImage *dynamic_image_into_luma16(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_into_luma8(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_into_luma_alpha16(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_into_luma_alpha8(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_into_rgb16(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_into_rgb32f(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_into_rgb8(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_into_rgba16(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_into_rgba32f(struct DynamicImage *this_);

struct DynamicImage *dynamic_image_into_rgba8(struct DynamicImage *this_);

enum ErrorType dynamic_image_write_to(struct DynamicImage *this_,
                                      struct RustWriter *w,
                                      enum ImageFormat format);

enum ErrorType dynamic_image_write_with_encoder(struct DynamicImage *this_,
                                                struct ImageEncoder *encoder);

struct Dimensions dynamic_image_dimensions(struct DynamicImage *this_);

struct Rgba dynamic_image_get_pixel(struct DynamicImage *this_, uint32_t x, uint32_t y);

bool dynamic_image_in_bounds(struct DynamicImage *this_, uint32_t x, uint32_t y);

struct RawIterator dynamic_image_pixels(struct DynamicImage *this_);

void dynamic_image_free(struct DynamicImage *this_);

struct PixelResult ____(void);

void *iter_next(struct RawIterator *s);

struct SizeHint iter_size_hint(struct RawIterator *s);

uintptr_t iter_count(struct RawIterator *s);

void *iter_last(struct RawIterator *s);

void *iter_nth(struct RawIterator *s, uintptr_t n);

struct RawIterator iter_step_by(struct RawIterator *s, uintptr_t step);

struct RawIterator iter_chain(struct RawIterator *s, struct RawIterator other);

struct RawIterator iter_zip(struct RawIterator *s, struct RawIterator other);

struct RawIterator iter_map(struct RawIterator *s, void (*f)(void*));

void iter_for_each(struct RawIterator *s, void (*f)(void*));

struct RawIterator iter_filter(struct RawIterator *s, bool (*predicate)(void*));

struct RawIterator iter_filter_map(struct RawIterator *s, struct RawIterator *(*f)(void*));

struct RawIterator iter_enumerate(struct RawIterator *s);

struct RawIterator iter_peekable(struct RawIterator *s);

struct RawIterator iter_skip_while(struct RawIterator *s, bool (*predicate)(void*));

struct RawIterator iter_take_while(struct RawIterator *s, bool (*predicate)(void*));

struct RawIterator iter_map_while(struct RawIterator *s, struct RawIterator *(*predicate)(void*));

struct RawIterator iter_skip(struct RawIterator *s, uintptr_t n);

struct RawIterator iter_take(struct RawIterator *s, uintptr_t n);

struct RawIterator iter_scan(struct RawIterator *s, void *initial_state, void *(*f)(void*, void*));

struct RawIterator iter_flat_map(struct RawIterator *s, struct RawIterator (*f)(void*));

struct RawIterator iter_fuse(struct RawIterator *s);

struct RawIterator iter_inspect(struct RawIterator *s, void (*f)(void*));

struct RawIterator iter_by_ref(struct RawIterator *s);

void **iter_collect(struct RawIterator *s, uintptr_t *size);

void *iter_fold(struct RawIterator *s, void *init, void *(*f)(void*, void*));

void *iter_reduce(struct RawIterator *s, void *(*f)(void*, void*));

bool iter_all(struct RawIterator *s, bool (*f)(void*));

bool iter_any(struct RawIterator *s, bool (*f)(void*));

void *iter_find(struct RawIterator *s, bool (*predicate)(void*));

void *iter_find_map(struct RawIterator *s, void *(*f)(void*));

uintptr_t *iter_position(struct RawIterator *s, bool (*predicate)(void*));

void *iter_max(struct RawIterator *s);

void *iter_min(struct RawIterator *s);

enum Ordering iter_cmp(struct RawIterator *s, struct RawIterator other);

enum Ordering *iter_partial_cmp(struct RawIterator *s, struct RawIterator other);

bool iter_eq(struct RawIterator *s, struct RawIterator other);

bool iter_ne(struct RawIterator *s, struct RawIterator other);

bool iter_lt(struct RawIterator *s, struct RawIterator other);

bool iter_le(struct RawIterator *s, struct RawIterator other);

bool iter_gt(struct RawIterator *s, struct RawIterator other);

bool iter_ge(struct RawIterator *s, struct RawIterator other);

#endif  /* __INTERNAL_IMAGE_LOAD_H */
