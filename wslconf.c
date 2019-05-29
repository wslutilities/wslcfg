#include "wslconf.h"
int main(int argc, char* argv[]) {
    if (getuid() != 0) {
        printf("%s: root privilege required. please run with sudo command.\n", argv[0]);
        return 1;
    } else {
        if (argc < 2) {
            print_help(argv[0]);
        } else {
            char* option = argv[1];
            /* get command */
            if (strcmp(option, "get") == 0) {

            }
            /* set command */
            else if (strcmp(option, "set") == 0) {

            }
            /* unset command */
            else if (strcmp(option, "unset") == 0) {

            }
            /* list command */
            else if (strcmp(option, "list") == 0) {

            }
        }
    }
    return 0;
}