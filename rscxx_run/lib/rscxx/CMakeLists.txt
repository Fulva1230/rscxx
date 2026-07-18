cmake_minimum_required(VERSION 4.0)

project(rscxx VERSION 0.1
        LANGUAGES CXX)

add_library(rscxx-dll SHARED IMPORTED)
set_target_properties(rscxx-dll PROPERTIES
        IMPORTED_IMPLIB ${CMAKE_CURRENT_LIST_DIR}/rscxx.dll.lib
        IMPORTED_LOCATION ${CMAKE_CURRENT_LIST_DIR}/rscxx.dll
)

add_library(rscxx INTERFACE)
target_include_directories(rscxx INTERFACE ${CMAKE_CURRENT_LIST_DIR}/include)
target_link_libraries(rscxx INTERFACE rscxx-dll ${CMAKE_CURRENT_LIST_DIR}/rscxx.lib ${CMAKE_CURRENT_LIST_DIR}/cxxbridge1.lib)