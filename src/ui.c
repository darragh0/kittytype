#include "ui.h"

#include <ncurses.h>


WINDOW* logo_win;
WINDOW* txt_win;


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

    // Can change @ runtime, hence creation of window here & not macro
    int rows, cols;
    getmaxyx(stdscr, rows, cols);

    // Don't allow top margin to nullify
    if (rows < LOGO_WIN_ROWS + LOGO_TOP_MARGIN || cols < LOGO_WIN_COLS) {
        display_resize_warning(rows, cols);
        refresh();
        return;
    }

    const int logo_total_margin = cols - LOGO_WIN_COLS;
    const int logo_left_margin = (logo_total_margin / 2) + (logo_total_margin % 2);

    printw("r_t=%d, c_t=%d, c_l=%d, m_l=%d", rows, cols, LOGO_WIN_COLS, logo_left_margin);
    logo_win = newwin(LOGO_WIN_ROWS, LOGO_WIN_COLS, LOGO_TOP_MARGIN, logo_left_margin);

    refresh();
    box(logo_win, 0, 0);

    for (int i = 0; i < LOGO_ROWS; i++)
        mvwprintw(logo_win, LOGO_PADDING + LOGO_BORDER_ENABLED + i, LOGO_PADDING + LOGO_BORDER_ENABLED, "%s", logo[i]);

    wrefresh(logo_win);
}

void cleanup_ui() {
    endwin();
}

void display_resize_warning(int rows, int cols) {
    static const int msglen = 20;

    const int x_pos = (cols - msglen) / 2;
    const int y_pos = (rows - 1) / 2;

    mvprintw(y_pos, x_pos, "Resize Window! >:(");
}
