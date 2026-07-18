cmake_minimum_required(VERSION 4.0)

project(rscxx_run VERSION 0.1
                  DESCRIPTION "Test C++ binding"
                  LANGUAGES CXX)
include(FetchContent)

add_subdirectory(lib/rscxx)

add_executable(rscxx_run src/main.cpp)
target_include_directories(rscxx_run PUBLIC include)
target_link_libraries(rscxx_run PUBLIC rscxx)

if(WIN32)
    add_custom_command(TARGET rscxx_run POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
            $<TARGET_RUNTIME_DLLS:rscxx_run>
            $<TARGET_FILE_DIR:rscxx_run>
            COMMAND_EXPAND_LISTS
    )
endif()