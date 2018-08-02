include_directories(${PROJECT_SOURCE_DIR}/src)

set(SD ${PROJECT_SOURCE_DIR}/src)
set(source
    example.c
    ${SD}/slmp.c )

add_executable(example ${source})

