find_path(SODIUM_INCLUDE_DIR sodium.h PATHS ${Sodium_DIR}/include)

if(WIN32)
    set(os windows)
elseif(APPLE)
    set(os apple)
else()
    set(os linux)
endif()

if (WIN32)
    if(CMAKE_SIZEOF_VOID_P EQUAL 8)
        set(architecture x64)
    elseif(CMAKE_SIZEOF_VOID_P EQUAL 4)
        set(architecture x86)
    endif()
    find_library(SODIUM_LIBRARY sodium NAMES libsodium PATHS
        ${Sodium_DIR}/${os}/${architecture}/dynamic/)
else()
    EXECUTE_PROCESS( COMMAND uname -m COMMAND tr -d '\n' OUTPUT_VARIABLE architecture)
    find_library(SODIUM_LIBRARY sodium NAMES libsodium PATHS
        ${Sodium_DIR}/${os}/${architecture}/)
endif()

include (FindPackageHandleStandardArgs)
find_package_handle_standard_args(Sodium DEFAULT_MSG SODIUM_INCLUDE_DIR SODIUM_LIBRARY)

if(Sodium_FOUND)
    set(Sodium_INCLUDE_DIRS ${SODIUM_INCLUDE_DIR})
    set(Sodium_LIBRARIES ${SODIUM_LIBRARY})
    message(STATUS "Found sodium: (include: ${Sodium_INCLUDE_DIRS}, 
    library: ${Sodium_LIBRARIES})")

    add_library(Sodium::Sodium SHARED IMPORTED)
    set_target_properties(Sodium::Sodium
        PROPERTIES
        INTERFACE_INCLUDE_DIRECTORIES ${SODIUM_INCLUDE_DIR}
        IMPORTED_LOCATION ${SODIUM_LIBRARY}
        IMPORTED_IMPLIB ${SODIUM_LIBRARY})

endif()
