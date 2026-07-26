get_filename_component(PACKAGE_PREFIX_DIR "${CMAKE_CURRENT_LIST_DIR}/../../.." ABSOLUTE)

add_library(rscxx-dll SHARED IMPORTED)
set_target_properties(rscxx-dll PROPERTIES
        IMPORTED_IMPLIB ${PACKAGE_PREFIX_DIR}/lib/rscxx.dll.lib
        IMPORTED_LOCATION ${PACKAGE_PREFIX_DIR}/bin/rscxx.dll
)

add_library(rscxx INTERFACE)
target_include_directories(rscxx INTERFACE ${PACKAGE_PREFIX_DIR}/include)
target_link_libraries(rscxx INTERFACE rscxx-dll ${PACKAGE_PREFIX_DIR}/lib/rscxx.lib ${PACKAGE_PREFIX_DIR}/lib/cxxbridge1.lib)