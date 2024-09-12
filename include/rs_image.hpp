#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <exception>
#include <functional>

namespace rs_image {
namespace internal {
extern "C" {
using std::abs;
#include "rs_image.h"
}
}  // namespace internal

using internal::ColorType;
using internal::Dimensions;
using internal::Ordering;
using internal::PixelResult;
using internal::Rgba;
using internal::SizeHint;

class image_error : public std::exception {
  internal::ErrorType er;

 public:
  image_error(internal::ErrorType er) : er(er) {};
  const char* what() const noexcept override;
};

template <class T>
class Iterator {
  internal::RawIterator* inner;

 public:
  Iterator(internal::RawIterator* inner) : inner(inner) {};

  SizeHint size_hint();
  uintptr_t count();
  Iterator step_by(uintptr_t step);
  Iterator chain(Iterator other);
  Iterator zip(Iterator other);
  Iterator map(void (*)(T));
  void for_each(void (*)(T));
  Iterator filter(bool (*)(T));
  Iterator filter_map(Iterator* (*)(T));
  Iterator enumerate();
  Iterator peekable();
  Iterator skip_while(bool (*)(T));
  Iterator take_while(bool (*)(T));
  Iterator map_while(Iterator* (*)(T));
  Iterator skip(uintptr_t n);
  Iterator take(uintptr_t n);
  Iterator scan(void* initial_state, void* (*)(void*, T));
  Iterator flat_map(Iterator (*)(T));
  Iterator fuse();
  Iterator inspect(void (*)(T));
  Iterator by_ref();
  bool all(bool (*)(T));
  bool any(bool (*)(T));
  Ordering cmp(Iterator other);
  bool eq(Iterator other);
  bool ne(Iterator other);
  bool lt(Iterator other);
  bool le(Iterator other);
  bool gt(Iterator other);
  bool ge(Iterator other);
  T next();
  T last();
  T nth(uintptr_t n);
  T fold(T init, T (*)(T, T));
  T reduce(T (*)(T, T));
  T find(bool (*)(T*));
  T find_map(T (*)(T));
  size_t* position(bool (*)(T*));
  T max();
  T min();
  Ordering* partial_cmp(Iterator* other);
  std::vector<T> collect();

  /*T* max_by_key(void* (*)(void*));
  T* max_by(Ordering (*)(void*, void*));
  T* min_by_key(void* (*)(void*));
  T* min_by(Ordering (*)(void*, void*));*/
};

class DynamicImage {
 private:
  internal::DynamicImage* img;
  DynamicImage(internal::DynamicImage* img) : img(img) {};

 public:
  uint32_t width();

  DynamicImage(std::vector<char> data);
  ~DynamicImage();

  // DynamicImage* from_decoder();
  uint8_t* as_bytes(size_t* count);
  DynamicImage* blur(float sigma);
  DynamicImage* brighten(float value);
  void invert();
  ColorType color();
  DynamicImage* fliph();
  DynamicImage* flipv();
  DynamicImage* grayscale();
  uint32_t height();
  DynamicImage* rotate180();
  DynamicImage* rotate270();
  DynamicImage* rotate90();
  DynamicImage* into_luma16();
  DynamicImage* into_luma8();
  DynamicImage* into_luma_alpha16();
  DynamicImage* into_luma_alpha8();
  DynamicImage* into_rgb16();
  DynamicImage* into_rgb32f();
  DynamicImage* into_rgb8();
  DynamicImage* into_rgba16();
  DynamicImage* into_rgba32f();
  DynamicImage* into_rgba8();
  Dimensions get_dimensions();
  Rgba get_pixel(uint32_t x, uint32_t y);
  bool in_bounds(uint32_t x, uint32_t y);
  Iterator<PixelResult*> pixels();
};

// Thank you C++ gods for appearently making it so C++ templates need to be
// defined in the header- why the fuck am I even using them anyways?
#ifndef __ITERATOR_IMPLEMENTED
#define __ITERATOR_IMPLEMENTED
template <class T>
SizeHint Iterator<T>::size_hint() {
  return *(SizeHint*)internal::iter_size_hint(this->inner);
}
template <class T>
uintptr_t Iterator<T>::count() {
  return *(uintptr_t*)internal::iter_count(this->inner);
}
template <class T>
Iterator<T> Iterator<T>::step_by(uintptr_t step) {
  return *(Iterator<T>*)internal::iter_step_by(this->inner, step);
}
template <class T>
Iterator<T> Iterator<T>::chain(Iterator other) {
  return *(Iterator<T>*)internal::iter_chain(this->inner, other);
}
template <class T>
Iterator<T> Iterator<T>::zip(Iterator other) {
  return *(Iterator<T>*)internal::iter_zip(this->inner, other);
}
template <class T>
Iterator<T> Iterator<T>::map(void (*f)(T)) {
  return *(Iterator<T>*)internal::iter_map(this->inner, f);
}

template <class T>
void Iterator<T>::for_each(void (*f)(T)) {
  return internal::iter_for_each(this->inner, f);
}

template <class T>
Iterator<T> Iterator<T>::filter(bool (*f)(T)) {
  return *(Iterator<T>*)internal::iter_filter(this->inner, f);
}
template <class T>
Iterator<T> Iterator<T>::filter_map(Iterator* (*f)(T)) {
  return *(Iterator<T>*)internal::iter_filter_map(this->inner, f);
}
template <class T>
Iterator<T> Iterator<T>::enumerate() {
  return *(Iterator<T>*)internal::iter_enumerate(this->inner);
}
template <class T>
Iterator<T> Iterator<T>::peekable() {
  return *(Iterator<T>*)internal::iter_peekable(this->inner);
}
template <class T>
Iterator<T> Iterator<T>::skip_while(bool (*f)(T)) {
  return *(Iterator<T>*)internal::iter_skip_while(this->inner, f);
}
template <class T>
Iterator<T> Iterator<T>::take_while(bool (*f)(T)) {
  return *(Iterator<T>*)internal::iter_take_while(this->inner, f);
}
template <class T>
Iterator<T> Iterator<T>::map_while(Iterator* (*f)(T)) {
  return *(Iterator<T>*)internal::iter_map_while(this->inner, f);
}
template <class T>
Iterator<T> Iterator<T>::skip(uintptr_t n) {
  return *(Iterator<T>*)internal::iter_skip(this->inner, n);
}
template <class T>
Iterator<T> Iterator<T>::take(uintptr_t n) {
  return *(Iterator<T>*)internal::iter_take(this->inner, n);
}
template <class T>
Iterator<T> Iterator<T>::scan(void* initial_state, void* (*f)(void*, T)) {
  return *(Iterator<T>*)internal::iter_scan(this->inner, initial_state, f);
}
template <class T>
Iterator<T> Iterator<T>::flat_map(Iterator (*f)(T)) {
  return *(Iterator<T>*)internal::iter_flat_map(this->inner, f);
}
template <class T>
Iterator<T> Iterator<T>::fuse() {
  return *(Iterator<T>*)internal::iter_fuse(this->inner);
}
template <class T>
Iterator<T> Iterator<T>::inspect(void (*f)(T)) {
  return *(Iterator<T>*)internal::iter_inspect(this->inner, f);
}
template <class T>
Iterator<T> Iterator<T>::by_ref() {
  return *(Iterator<T>*)internal::iter_by_ref(this->inner);
}
template <class T>
bool Iterator<T>::all(bool (*f)(T)) {
  return internal::iter_all(this->inner, f);
}
template <class T>
bool Iterator<T>::any(bool (*f)(T)) {
  return internal::iter_any(this->inner, f);
}
template <class T>
Ordering Iterator<T>::cmp(Iterator other) {
  return internal::iter_cmp(this->inner, other);
}
template <class T>
bool Iterator<T>::eq(Iterator other) {
  return *(Iterator<T>*)internal::iter_eq(this->inner, other);
}
template <class T>
bool Iterator<T>::ne(Iterator other) {
  return *(Iterator<T>*)internal::iter_ne(this->inner, other);
}
template <class T>
bool Iterator<T>::lt(Iterator other) {
  return *(Iterator<T>*)internal::iter_lt(this->inner, other);
}
template <class T>
bool Iterator<T>::le(Iterator other) {
  return *(Iterator<T>*)internal::iter_le(this->inner, other);
}
template <class T>
bool Iterator<T>::gt(Iterator other) {
  return *(Iterator<T>*)internal::iter_gt(this->inner, other);
}
template <class T>
bool Iterator<T>::ge(Iterator other) {
  return *(Iterator<T>*)internal::iter_ge(this->inner, other);
}
template <class T>
T Iterator<T>::next() {
  return *(T*)internal::iter_next(this->inner);
}
template <class T>
T Iterator<T>::last() {
  return *(T*)internal::iter_last(this->inner);
}
template <class T>
T Iterator<T>::nth(uintptr_t n) {
  return *(T*)internal::iter_nth(this->inner, n);
}
template <class T>
T Iterator<T>::fold(T init, T (*f)(T, T)) {
  return *(T*)internal::iter_fold(this->inner, init, f);
}
template <class T>
T Iterator<T>::reduce(T (*f)(T, T)) {
  return *(T*)internal::iter_reduce(this->inner, f);
}
template <class T>
T Iterator<T>::find(bool (*f)(T*)) {
  return *(T*)internal::iter_find(this->inner, f);
}
template <class T>
T Iterator<T>::find_map(T (*f)(T)) {
  return *(T*)internal::iter_find_map(this->inner, f);
}
template <class T>
size_t* Iterator<T>::position(bool (*f)(T*)) {
  return *(size_t*)internal::iter_position(this->inner, f);
}
template <class T>
T Iterator<T>::max() {
  return *(T*)internal::iter_max(this->inner);
}
template <class T>
T Iterator<T>::min() {
  return *(T*)internal::iter_min(this->inner);
}
template <class T>
Ordering* Iterator<T>::partial_cmp(Iterator* other) {
  return internal::iter_partial_cmp(this->inner, other);
}
template <class T>
std::vector<T> Iterator<T>::collect() {
  uintptr_t size;
  auto bytes = internal::iter_collect(this->inner, &size);
  return std::vector(bytes, bytes + size);
}
#endif

}  // namespace rs_image
