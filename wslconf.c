#include "wslconf.h"
int main(int argc, char* argv[]) {
    if (getuid() != 0) {
        printf("%s: requires sudo. please run with sudo command.\n", argv[0]);
    } else {
        if (argc < 2) {
            print_help(argv[0]);
        }
    }
    return 0;
}