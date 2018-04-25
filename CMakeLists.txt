# ------------------------------------------------------------------------------
# Project
# ------------------------------------------------------------------------------
cmake_minimum_required(VERSION 3.0)

project(lc)


# ------------------------------------------------------------------------------
# Source files
# ------------------------------------------------------------------------------
set(COMMON_SRC
  include/map.hpp
  include/component.hpp
  include/entity.hpp
  include/floodfill.hpp
  include/game.hpp
  include/geometry.hpp
  include/gui.hpp
  include/io.hpp
  include/main_menu.hpp
  include/pathfinding.hpp
  include/state.hpp

  src/map.cpp
  src/floodfill.cpp
  src/game.cpp
  src/geometry.cpp
  src/gui.cpp
  src/main_menu.cpp
  src/pathfinding.cpp
  src/state.cpp
  )

set(GAME_SRC
  ${COMMON_SRC}

  src/main.cpp
  src/io.cpp
  )

set(TEST_SRC
  ${COMMON_SRC}

  test/include/pathfinding_tests.hpp
  test/include/floodfill_tests.hpp
  test/include/gui_tests.hpp

  test/src/pathfinding_tests.cpp
  test/src/floodfill_tests.cpp
  test/src/gui_tests.cpp

  test/src/main.cpp

  test/stubs/src/io.cpp
  )

# ------------------------------------------------------------------------------
# Target definitions
# ------------------------------------------------------------------------------
add_executable(lc       ${GAME_SRC})
add_executable(lc-debug ${GAME_SRC})
add_executable(lc-test  ${TEST_SRC})

set_target_properties(lc        PROPERTIES OUTPUT_NAME lc)
set_target_properties(lc-debug  PROPERTIES OUTPUT_NAME lc-debug)

# TODO: Enable this flag:
# -Wfloat-conversion

# GNU gcc and Clang specific compiler flags
if(CMAKE_COMPILER_IS_GNUCXX OR(CMAKE_C_COMPILER_ID MATCHES "Clang") OR(CMAKE_CXX_COMPILER_ID MATCHES "Clang"))

  set(COMMON_COMPILE_FLAGS
    -std=c++14
    -fno-rtti
    -Wpedantic
    -Wall
    -Wextra
    -Werror
    -Weffc++
    )

  set(DEBUG_COMPILE_FLAGS
    -fno-exceptions
    -O0
    -g
    # Uncomment to generate gmon.out for gprof (must also be done for linker
    # flags, see below)
    # -pg
    )

  set(RELEASE_COMPILE_FLAGS
    -fno-exceptions
    -O2
    -Wno-unused-value
    )

endif()

set(RELEASE_COMPILE_FLAGS
    ${RELEASE_COMPILE_FLAGS}
    -DNDEBUG
    )

target_compile_options(lc PUBLIC
    ${COMMON_COMPILE_FLAGS}
    ${RELEASE_COMPILE_FLAGS}
    )

target_compile_options(lc-debug PUBLIC
    ${COMMON_COMPILE_FLAGS}
    ${DEBUG_COMPILE_FLAGS}
    )

target_compile_options(lc-test PUBLIC
    ${COMMON_COMPILE_FLAGS}
    ${DEBUG_COMPILE_FLAGS}
    )

set(COMMON_INCLUDE_DIRS
    include
    )

target_include_directories(lc PUBLIC
    ${COMMON_INCLUDE_DIRS}
    )

target_include_directories(lc-debug PUBLIC
    ${COMMON_INCLUDE_DIRS}
    )

target_include_directories(lc-test PUBLIC
    ${COMMON_INCLUDE_DIRS}
    test/include
    )

# On Windows releases, remove the console window
if(WIN32)
  # TODO: This solution only works with gcc/clang - not with MSVC
  if(CMAKE_COMPILER_IS_GNUCXX OR(CMAKE_C_COMPILER_ID MATCHES "Clang") OR(CMAKE_CXX_COMPILER_ID MATCHES "Clang"))
    target_link_libraries(lc -mwindows)
  endif()
  if(CMAKE_COMPILER_IS_GNUCXX)
    target_link_libraries(lc -static-libgcc -static-libstdc++)
  endif()
endif()

# Copy all resource files to the build directory.
file(COPY data DESTINATION .)
file(COPY font DESTINATION .)
file(COPY gfx DESTINATION .)


# ------------------------------------------------------------------------------
# Dependencies
# ------------------------------------------------------------------------------
if(WIN32)

    if (NOT DEFINED ARCH)

        if("${CMAKE_SIZEOF_VOID_P}" EQUAL "4")
            message(STATUS "Assuming 32 bit architecture")
            set(ARCH 32bit)
        elseif("${CMAKE_SIZEOF_VOID_P}" EQUAL "8")
            message(STATUS "Assuming 64 bit architecture")
            set(ARCH 64bit)
        endif()

    endif()

    if (NOT DEFINED ARCH OR ((NOT "${ARCH}" EQUAL "32bit") AND (NOT "${ARCH}" EQUAL "64bit")))
        message(FATAL_ERROR "Unknown architecture")
    endif()

    set(SDL_BASE_DIR ${CMAKE_SOURCE_DIR}/SDL)

    if(MSVC)

        if("${ARCH}" EQUAL "32bit")
            set(SDL_ARCH_DIR x86)
        else()
            set(SDL_ARCH_DIR x64)
        endif()

        set(SDL2_PATH       ${SDL_BASE_DIR}/msvc/SDL2-2.0.5)
        set(SDL2_IMAGE_PATH ${SDL_BASE_DIR}/msvc/SDL2_image-2.0.1)
        set(SDL2_MIXER_PATH ${SDL_BASE_DIR}/msvc/SDL2_mixer-2.0.1)

        set(SDL_INCLUDE_DIRS
            ${SDL2_PATH}/include
            ${SDL2_IMAGE_PATH}/include
            ${SDL2_MIXER_PATH}/include
            )

        set(SDL2_LIBS_PATH          ${SDL2_PATH}/lib/${SDL_ARCH_DIR})
        set(SDL2_IMAGE_LIBS_PATH    ${SDL2_IMAGE_PATH}/lib/${SDL_ARCH_DIR})
        set(SDL2_MIXER_LIBS_PATH    ${SDL2_MIXER_PATH}/lib/${SDL_ARCH_DIR})

        set(SDL2_BINS_PATH          ${SDL2_PATH}/lib/${SDL_ARCH_DIR})
        set(SDL2_IMAGE_BINS_PATH    ${SDL2_IMAGE_PATH}/lib/${SDL_ARCH_DIR})
        set(SDL2_MIXER_BINS_PATH    ${SDL2_MIXER_PATH}/lib/${SDL_ARCH_DIR})

    else()

        # Not MSVC - e.g. gcc

        if("${ARCH}" EQUAL "32bit")
            set(SDL_ARCH_DIR i686-w64-mingw32)
        else()
            set(SDL_ARCH_DIR x86_64-w64-mingw32)
        endif()

        set(SDL2_PATH       ${SDL_BASE_DIR}/mingw/SDL2-2.0.5/${SDL_ARCH_DIR})
        set(SDL2_IMAGE_PATH ${SDL_BASE_DIR}/mingw/SDL2_image-2.0.1/${SDL_ARCH_DIR})
        set(SDL2_MIXER_PATH ${SDL_BASE_DIR}/mingw/SDL2_mixer-2.0.1/${SDL_ARCH_DIR})

        set(SDL_INCLUDE_DIRS
            ${SDL2_PATH}/include/SDL2
            ${SDL2_IMAGE_PATH}/include/SDL2
            ${SDL2_MIXER_PATH}/include/SDL2
            )

        set(SDL2_LIBS_PATH          ${SDL2_PATH}/lib)
        set(SDL2_IMAGE_LIBS_PATH    ${SDL2_IMAGE_PATH}/lib)
        set(SDL2_MIXER_LIBS_PATH    ${SDL2_MIXER_PATH}/lib)

        set(SDL2_BINS_PATH          ${SDL2_PATH}/bin)
        set(SDL2_IMAGE_BINS_PATH    ${SDL2_IMAGE_PATH}/bin)
        set(SDL2_MIXER_BINS_PATH    ${SDL2_MIXER_PATH}/bin)

        target_link_libraries(lc        mingw32)
        target_link_libraries(lc-debug  mingw32)

    endif()

    message(STATUS "SDL_INCLUDE_DIRS: " ${SDL_INCLUDE_DIRS})

    target_include_directories(lc       PUBLIC ${SDL_INCLUDE_DIRS})
    target_include_directories(lc-debug PUBLIC ${SDL_INCLUDE_DIRS})

    message(STATUS "SDL2_LIBS_PATH: "        ${SDL2_LIBS_PATH})
    message(STATUS "SDL2_IMAGE_LIBS_PATH: "  ${SDL2_IMAGE_LIBS_PATH})
    message(STATUS "SDL2_MIXER_LIBS_PATH: "  ${SDL2_MIXER_LIBS_PATH})

    find_library(SDL2_LIB_PATH          SDL2        PATHS ${SDL2_LIBS_PATH})
    find_library(SDL2_MAIN_LIB_PATH     SDL2main    PATHS ${SDL2_LIBS_PATH})
    find_library(SDL2_IMAGE_LIB_PATH    SDL2_image  PATHS ${SDL2_IMAGE_LIBS_PATH})
    find_library(SDL2_MIXER_LIB_PATH    SDL2_mixer  PATHS ${SDL2_MIXER_LIBS_PATH})

    message(STATUS "SDL2_LIB_PATH: "        ${SDL2_LIB_PATH})
    message(STATUS "SDL2_MAIN_LIB_PATH: "   ${SDL2_MAIN_LIB_PATH})
    message(STATUS "SDL2_IMAGE_LIB_PATH: "  ${SDL2_IMAGE_LIB_PATH})
    message(STATUS "SDL2_MIXER_LIB_PATH: "  ${SDL2_MIXER_LIB_PATH})

    set(SDL_LIBS
        ${SDL2_LIB_PATH}
        ${SDL2_MAIN_LIB_PATH}
        ${SDL2_IMAGE_LIB_PATH}
        ${SDL2_MIXER_LIB_PATH}
        )

    target_link_libraries(lc        ${SDL_LIBS})
    target_link_libraries(lc-debug  ${SDL_LIBS})

    # SDL dll files and licenses
    set(SDL_DISTR_FILES
        ${SDL2_BINS_PATH}/SDL2.dll
        ${SDL2_IMAGE_BINS_PATH}/SDL2_image.dll
        ${SDL2_IMAGE_BINS_PATH}/zlib1.dll
        ${SDL2_IMAGE_BINS_PATH}/libpng16-16.dll
        ${SDL2_IMAGE_BINS_PATH}/LICENSE.zlib.txt
        ${SDL2_IMAGE_BINS_PATH}/LICENSE.png.txt
        ${SDL2_MIXER_BINS_PATH}/SDL2_mixer.dll
        ${SDL2_MIXER_BINS_PATH}/libogg-0.dll
        ${SDL2_MIXER_BINS_PATH}/libvorbis-0.dll
        ${SDL2_MIXER_BINS_PATH}/libvorbisfile-3.dll
        ${SDL2_MIXER_BINS_PATH}/LICENSE.ogg-vorbis.txt
        )

    file(COPY ${SDL_DISTR_FILES} DESTINATION .)

    install(FILES ${SDL_DISTR_FILES} DESTINATION lc)
    install(FILES ${SDL_DISTR_FILES} DESTINATION lc-debug)

else()

    # Not windows (e.g. Unix)

    set(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake)

    find_package(SDL2       REQUIRED)
    find_package(SDL2_image REQUIRED)
    find_package(SDL2_mixer REQUIRED)
    find_package(SDL2_ttf   REQUIRED)

    set(SDL_INCLUDE_DIRS
        ${SDL2_INCLUDE_DIR}
        ${SDL2_IMAGE_INCLUDE_DIR}
        ${SDL2_MIXER_INCLUDE_DIR}
        ${SDL2_TTF_INCLUDE_DIR}
        )

    target_include_directories(lc       PUBLIC ${SDL_INCLUDE_DIRS})
    target_include_directories(lc-debug PUBLIC ${SDL_INCLUDE_DIRS})

    set(SDL_LIBS
        ${SDL2_LIBRARY}
        ${SDL2_IMAGE_LIBRARIES}
        ${SDL2_MIXER_LIBRARIES}
        ${SDL2_TTF_LIBRARIES}
        )

    target_link_libraries(lc PUBLIC         ${SDL_LIBS})
    target_link_libraries(lc-debug PUBLIC   ${SDL_LIBS})

    # Uncomment to generate gmon.out for gprof (must also be done for compile
    # flags, see above)
    # NOTE: "-no-pie" is needed due to "a bug in gcc", see:
    # https://stackoverflow.com/questions/42620074
    # https://stackoverflow.com/questions/39827412
    # target_link_libraries(lc-debug PUBLIC -pg -no-pie)

endif()


# ------------------------------------------------------------------------------
# Packaging
# ------------------------------------------------------------------------------
set(CMAKE_INSTALL_PREFIX target)

install(TARGETS lc       DESTINATION lc)
install(TARGETS lc-debug DESTINATION lc-debug)

# TODO: There should be a better way to handle this...
install(DIRECTORY data DESTINATION lc)
install(DIRECTORY font DESTINATION lc)
install(DIRECTORY gfx  DESTINATION lc)

install(DIRECTORY data DESTINATION lc-debug)
install(DIRECTORY font DESTINATION lc-debug)
install(DIRECTORY gfx  DESTINATION lc-debug)
