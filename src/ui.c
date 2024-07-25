#include "ui.h"

#include <ncurses.h>


WINDOW *title;

const char logo[LOGO_ROWS][LOGO_COLS + 1] = {
    "    __   _ __  __        __                 ",
    "   / /__(_) /_/ /___  __/ /___  ______  ___ ",
    "  / //_/ / __/ __/ / / / __/ / / / __ \\/ _ \\",
    " / ,< / / /_/ /_/ /_/ / /_/ /_/ / /_/ /  __/",
    "/_/|_/_/\\__/\\__/\\__, /\\__/\\__, / .___/\\___/ ",
    "               /____/    /____/_/           "
};

void setup_ui() {
    initscr();
    curs_set(0);
    noecho();
    refresh_ui();
}

void refresh_ui() {

    clear();

    int rows, cols;
    getmaxyx(stdscr, rows, cols);

    // Allow left margin to nullify
    if (rows < LOGO_ROWS + LOGO_TOP_MARGIN || cols < LOGO_COLS) {
        refresh();
        return;
    }

    // Can change at runtime, hence no macro and creation of window here
    const int logo_left_margin = (cols - LOGO_COLS) / 2;
    title = newwin(LOGO_ROWS, LOGO_COLS, LOGO_TOP_MARGIN, logo_left_margin);

    refresh();

    for (int i = 0; i < LOGO_ROWS; i++)
        wprintw(title, "%s", logo[i]);

    wrefresh(title);
}

void cleanup_ui() {
    endwin();
}
