# rs_image

Bindings for Rust's image crate to C/C++. Rust's image crate, for coincidental reasons that I'm unsure of, supports way more image formats (including the one I needed for a project) then what I could find in C/C++. 

| Format | Decoding | Encoding |
| -------- | -------- | -------- | 
| AVIF | Yes (8-bit only) * | Yes (lossy only) |
| BMP | Yes | Yes |
| DDS | Yes | --- |
| Farbfeld | Yes | Yes |
| GIF | Yes | Yes |
| HDR | Yes | Yes |
| ICO | Yes | Yes |
| JPEG | Yes | Yes |
| EXR | Yes | Yes |
| PNG | Yes | Yes |
| PNM | Yes | Yes |
| QOI | Yes | Yes |
| TGA | Yes | Yes
| TIFF | Yes | Yes |
| WebP | Yes | Yes (lossless only) |

* If CMake is able to find `libdav1d` on your system.

Currently, the most important parts - `DynamicImage`, and Rust's own `std::iter::Iterator`, are bound. 

# Usage

C++
```cxx
// Currently, only loading from your own data buffer is supported.
std::ifstream is("image.png");
std::istream_iterator<char> start(is), end;
std::vector<char> image(start, end);

auto img = new DynamicImage(image);
std::println(std::cout, "loaded a {}x{} image", img.width(),
                img.height());

auto pixels = img.pixels();

for (auto pixel = pixels.next(); pixel != NULL; pixel = pixels.next()) {
    auto color = pixel->color;
    std::println(std::cout, "At ({} {}): ({} {} {} {})", pixel->x, pixel->y,color.r,color.g,color.b,color.a);
}
```

C
```c
// have fun!
```

# Installation

It can be imported via cmake:

```cmake
FetchContent_Declare(
    rs_systemtime
    GIT_REPOSITORY "https://github.com/IoIxD/rs_image.git"
    GIT_PROGRESS TRUE
)
FetchContent_MakeAvailable(rs_image)
include_directories(${RS_IMAGE_INCLUDE_DIRECTORY})
```