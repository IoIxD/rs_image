#include <sys/types.h>
#include <cstdint>
#include <cstdlib>
#include <vector>

#include "rs_image.hpp"

// We do this so that I can use tab complete without clangd automatically
// importing the other file
#ifdef NEVER_DEFINED
#include "rs_image.h"
#endif

namespace rs_image {

uint32_t DynamicImage::width() {
  return internal::dynamic_image_width(this->img);
};

DynamicImage::DynamicImage(std::vector<char> data) {
  auto er = internal::dynamic_image_load_from_memory(
      (uint8_t*)(char*)data.data(), data.size());
  if (er.err != internal::IMAGE_ERROR_NONE) {
    throw new image_error(er.err);
  }
  this->img = er.res;
};

DynamicImage::~DynamicImage() {
  internal::dynamic_image_free(this->img);
};
Dimensions DynamicImage::get_dimensions() {
  return internal::dynamic_image_dimensions(this->img);
};

Rgba DynamicImage::get_pixel(uint32_t x, uint32_t y) {
  return internal::dynamic_image_get_pixel(this->img, x, y);
};

uint32_t DynamicImage::height() {
  return internal::dynamic_image_height(this->img);
};

bool DynamicImage::in_bounds(uint32_t x, uint32_t y) {
  return internal::dynamic_image_in_bounds(this->img, x, y);
};

Iterator<PixelResult*> DynamicImage::pixels() {
  return new internal::RawIterator(internal::dynamic_image_pixels(this->img));
}

DynamicImage* DynamicImage::blur(float sigma) {
  return new DynamicImage(internal::dynamic_image_blur(this->img, sigma));
};
DynamicImage* DynamicImage::brighten(float value) {
  return new DynamicImage(internal::dynamic_image_brighten(this->img, value));
}
void DynamicImage::invert() {
  internal::dynamic_image_invert(this->img);
}
internal::ColorType DynamicImage::color() {
  return internal::dynamic_image_color(this->img);
}
DynamicImage* DynamicImage::fliph() {
  return new DynamicImage(internal::dynamic_image_fliph(this->img));
}
DynamicImage* DynamicImage::flipv() {
  return new DynamicImage(internal::dynamic_image_flipv(this->img));
}
DynamicImage* DynamicImage::grayscale() {
  return new DynamicImage(internal::dynamic_image_grayscale(this->img));
}
DynamicImage* DynamicImage::rotate180() {
  return new DynamicImage(internal::dynamic_image_rotate180(this->img));
}
DynamicImage* DynamicImage::rotate270() {
  return new DynamicImage(internal::dynamic_image_rotate270(this->img));
}
DynamicImage* DynamicImage::rotate90() {
  return new DynamicImage(internal::dynamic_image_rotate90(this->img));
}
DynamicImage* DynamicImage::into_luma16() {
  return new DynamicImage(internal::dynamic_image_into_luma16(this->img));
}
DynamicImage* DynamicImage::into_luma8() {
  return new DynamicImage(internal::dynamic_image_into_luma8(this->img));
}
DynamicImage* DynamicImage::into_luma_alpha16() {
  return new DynamicImage(internal::dynamic_image_into_luma_alpha16(this->img));
}
DynamicImage* DynamicImage::into_luma_alpha8() {
  return new DynamicImage(internal::dynamic_image_into_luma8(this->img));
}
DynamicImage* DynamicImage::into_rgb16() {
  return new DynamicImage(internal::dynamic_image_into_rgb16(this->img));
}
DynamicImage* DynamicImage::into_rgb32f() {
  return new DynamicImage(internal::dynamic_image_into_rgb32f(this->img));
}
DynamicImage* DynamicImage::into_rgb8() {
  return new DynamicImage(internal::dynamic_image_into_rgb8(this->img));
}
DynamicImage* DynamicImage::into_rgba16() {
  return new DynamicImage(internal::dynamic_image_into_rgba16(this->img));
}
DynamicImage* DynamicImage::into_rgba32f() {
  return new DynamicImage(internal::dynamic_image_into_rgba32f(this->img));
}
DynamicImage* DynamicImage::into_rgba8() {
  return new DynamicImage(internal::dynamic_image_into_rgba8(this->img));
}
}  // namespace rs_image