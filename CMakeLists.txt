cmake_minimum_required(VERSION 3.21)
project(rs_image)

FILE(GLOB sources src/*.cpp)

include_directories(include)

add_library(${PROJECT_NAME} ${sources})

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

set(RS_IMAGE_INCLUDE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/include CACHE INTERNAL "RS_IMAGE_INCLUDE_DIRECTORY")

set(CRATE_FEATURES "")

pkg_check_modules(DAV1D REQUIRED dav1d)

if(${DAV1D_FOUND})
    message(STATUS "libdav1d found, enabling avif decoding.")
    set(CRATE_FEATURES "avif") 
    target_link_libraries(${PROJECT_NAME} dav1d)
endif()

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH Cargo.toml FEATURES ${CRATE_FEATURES})
target_link_libraries(${PROJECT_NAME} rs_image_internal)