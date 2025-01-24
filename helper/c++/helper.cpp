#include "helper.hpp"
#include <cstdlib>

std::string Helper::get_input_path(const char id) {
    const char* input_dir = std::getenv("AOC_INPUT_DIR");
    if (input_dir) {
        return std::string(input_dir) + "/" + id + ".txt";
    }
    return "input.txt";  // Fallback
}
