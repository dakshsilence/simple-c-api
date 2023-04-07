project(RNSIMPLE)
cmake_minimum_required(VERSION 3.4.1)

set (CMAKE_VERBOSE_MAKEFILE ON)
set (CMAKE_CXX_STANDARD 11)

set (PACKAGE_NAME "rnsimple")
message("-- ABI     : " ${ANDROID_ABI})
message("-- CMAKE DIR   :",${CMAKE_LIBRARY_OUTPUT_DIRECTORY})

add_library(
  ${PACKAGE_NAME}
  SHARED
  cpp-adapter.cpp
)

# Specifies a path to native header files.
include_directories(
  ../csimple
)

set (SIMPLE_LIB_PATH "${CMAKE_CURRENT_SOURCE_DIR}/../libs/android/${ANDROID_ABI}")

target_link_libraries(
  ${PACKAGE_NAME}
  -L${SIMPLE_LIB_PATH} -lsimple
)

add_custom_command(
  TARGET ${PACKAGE_NAME} POST_BUILD
  COMMAND cp ${CMAKE_CURRENT_SOURCE_DIR}/../libs/android/${ANDROID_ABI}/libsimple.so ${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/libsimple.so
)
