//"KB handling.h"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <termios.h>

struct termios term_settings, term_settings_saved;

int getchec() {
    int temp;
    if (tcgetattr(STDIN_FILENO, &term_settings))
        return -1;
    term_settings_saved = term_settings;
    term_settings.c_lflag &= ~ICANON;
    term_settings.c_cc[VMIN] = 1;
    term_settings.c_cc[VTIME] = 0;
    if (tcsetattr(STDIN_FILENO, TCSANOW, &term_settings) < 0) {
        tcsetattr(STDIN_FILENO, TCSANOW, &term_settings_saved);
        return -1;
    }
    temp = getchar();
    tcsetattr(STDIN_FILENO, TCSANOW, &term_settings_saved);
    return temp;
}

int getchnec() {
    int temp;
    if (tcgetattr(STDIN_FILENO, &term_settings))
        return -1;
    term_settings_saved = term_settings;
    term_settings.c_lflag &= ~ICANON;
    term_settings.c_lflag &= ~ECHO;
    term_settings.c_cc[VMIN] = 1;
    term_settings.c_cc[VTIME] = 0;
    if (tcsetattr(STDIN_FILENO, TCSANOW, &term_settings) < 0) {
        tcsetattr(STDIN_FILENO, TCSANOW, &term_settings_saved);
        return -1;
    }
    temp = getchar();
    tcsetattr(STDIN_FILENO, TCSANOW, &term_settings_saved);
    return temp;
}


int keyhit() {
    if (tcgetattr(STDIN_FILENO, &term_settings))
        return false;
    term_settings_saved = term_settings;
    term_settings.c_lflag &= ~ICANON;
    term_settings.c_cc[VMIN] = 0;
    term_settings.c_cc[VTIME] = 0;
    if (tcsetattr(STDIN_FILENO, TCSANOW, &term_settings) < 0) {
        tcsetattr(STDIN_FILENO, TCSANOW, &term_settings_saved);
        return false;
    }
    fd_set obscureFileDescriptorThing;
    FD_SET(STDIN_FILENO, &obscureFileDescriptorThing);
    struct timeval TimeZero;
    TimeZero.tv_sec = 0;
    TimeZero.tv_usec = 0;
    int test = select(1, &obscureFileDescriptorThing, 0, 0, &TimeZero);

    tcsetattr(STDIN_FILENO, TCSANOW, &term_settings_saved);
    if (test == 1)
        return true;
    else
        return false;
}


/*
#include <stdio.h>
#include "KB handling.h"

main() {
    int i = 0;
    int n = 0;
    while (i != 'O') {
        if (keyhit()) {
            printf("Tiger goes %d times: %c\n", n, (i = getchnec()));
        } else
            n++;
    }
}
*/
