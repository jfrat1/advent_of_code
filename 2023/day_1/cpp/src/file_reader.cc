#include <sstream>

#include "file_reader.h"

FileManager::FileManager(FILE* file_pointer) {
    this->file_pointer = file_pointer;
}

FileManager::~FileManager() {
    fclose(this->file_pointer);
}

std::string FileManager::FileData() const {
    std::fseek(this->file_pointer, 0, SEEK_END);
    size_t file_size = std::ftell(this->file_pointer);
    std::rewind(this->file_pointer);

    std::string data;
    data.resize(file_size);

    std::fread(&data[0], 1, data.size(), this->file_pointer);
    std::rewind(this->file_pointer);

    return(data);
}

FileReader::FileReader(const IFileManager* file_manager) {
    this->file_manager = file_manager;
}

FileLines FileReader::ReadLines() const {
    std::string file_data = this->file_manager->FileData();
    std::istringstream stream(file_data);

    std::string line;
    FileLines lines {};

    while(std::getline(stream, line)) {
        lines.push_back(line);
    }
    return lines;
}
