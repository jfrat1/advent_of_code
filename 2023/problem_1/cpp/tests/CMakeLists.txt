enable_testing()


add_executable(
    file_reader_test
    file_reader_test.cc
)
target_link_libraries(
    file_reader_test
    file_reader
    GTest::gtest_main
)

include(GoogleTest)
gtest_discover_tests(file_reader_test)