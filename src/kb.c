#include "kb.h"
#include "ui.h"

#include <ncurses.h>


void mainloop() {

    int ch;
    while ((ch = getch()) != KEY_EXIT) {
        if (ch == KEY_RESIZE)
            refresh_ui();
    }

}
