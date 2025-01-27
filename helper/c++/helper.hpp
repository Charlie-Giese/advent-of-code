#ifndef __HELPER__
#define __HELPER__

#include <string>


class Helper {
    public:
        
        std::string get_input_path(const char* id);
        static char* fetch_line(const char* filename, int* lineno);
};

#endif
