enable_testing()

set(SOURCE_DATA_FILE_PATH  "${PROJECT_SOURCE_DIR}/tests/data/file_reader_test_data.txt")
set(BUILD_DATA_FOLDER_PATH  "${PROJECT_SOURCE_DIR}/build/tests/data")
file(COPY "${SOURCE_DATA_FILE_PATH}" DESTINATION "${BUILD_DATA_FOLDER_PATH}")

add_executable(
    file_reader_test
    file_reader_test.cc
)
target_link_libraries(
    file_reader_test
    file_reader
    GTest::gtest_main
    GTest::gmock_main
)

add_executable(
    solver_base_test
    solver_base_test.cc
)
target_link_libraries(
    solver_base_test
    solver_base
    GTest::gtest_main
    GTest::gmock_main
)

add_executable(
    game_test
    game_test.cc
)
target_link_libraries(
    game_test
    game
    GTest::gtest_main
    GTest::gmock_main
)


include(GoogleTest)
gtest_discover_tests(file_reader_test)
gtest_discover_tests(solver_base_test)
gtest_discover_tests(game_test)