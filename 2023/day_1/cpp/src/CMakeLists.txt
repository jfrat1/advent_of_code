add_library(
    file_reader
    file_reader.cc
)
target_include_directories(
    file_reader
    PUBLIC ../include
)

add_library(
    solver_base
    solver_base.cc
)
target_include_directories(
    solver_base
    PUBLIC ../include
)
target_link_libraries(
    solver_base
    file_reader
)

add_library(
    problem_1
    problem_1.cc
)
target_include_directories(
    problem_1
    PUBLIC ../include
)
target_link_libraries(
    problem_1
    file_reader
    solver_base
)

add_library(
    problem_2
    problem_2.cc
)
target_include_directories(
    problem_2
    PUBLIC ../include
)
target_link_libraries(
    problem_2
    file_reader
    solver_base
)

add_executable(
    main
    main.cc
)
target_link_libraries(
    main
    problem_1
    problem_2
    file_reader
)

set(SOURCE_DATA_FILE_PATH  "${PROJECT_SOURCE_DIR}/data/problem_data.txt")
set(BUILD_DATA_FOLDER_PATH  "${PROJECT_SOURCE_DIR}/build/data")
file(COPY "${SOURCE_DATA_FILE_PATH}" DESTINATION "${BUILD_DATA_FOLDER_PATH}")
