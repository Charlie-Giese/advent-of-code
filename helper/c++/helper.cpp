#include "helper.hpp"
#include <cstdlib>
#include <iostream>

std::string Helper::get_input_path(const char* id) {
    const char* input_dir = std::getenv("AOC_INPUT_DIR");
    std::cout << input_dir << std::endl;
    if (input_dir) {
        return std::string(input_dir) + "/" + id + ".txt";
    }
    return "input.txt";  // Fallback
}

char* Helper::fetch_line(const char* filename, int* lineno) {
    static std::string line;
    std::ifstream file(filename);

    if (!file.is_open()) {
        std::cerr << "Error: Unable to open file " << filename << std::endl;
        return nullptr;
    }

    if (std::getline(file, line)) {
        (*lineno)++;

        // Trim leading whitespace
        size_t start = line.find_first_not_of(" \t\r\n");
        // Trim trailing whitespace
        size_t end = line.find_last_not_of(" \t\r\n");

        if (start == std::string::npos) {
            line.clear();  // If the line contains only whitespace, return an empty string
        } else {
            line = line.substr(start, end - start + 1);
        }

        return &line[0];
    }

    return nullptr;
}

