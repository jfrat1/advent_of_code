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
    game
    game.cc
)
target_include_directories(
    game
    PUBLIC ../include
)

add_executable(
    main
    main.cc
)
target_link_libraries(
    main
    # problem_1
    # problem_2
    file_reader
)

set(SOURCE_DATA_FILE_PATH  "${PROJECT_SOURCE_DIR}/data/problem_data.txt")
set(BUILD_DATA_FOLDER_PATH  "${PROJECT_SOURCE_DIR}/build/data")
file(COPY "${SOURCE_DATA_FILE_PATH}" DESTINATION "${BUILD_DATA_FOLDER_PATH}")
