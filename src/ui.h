#ifndef __H_UI_H__
#define __H_UI_H__

#include <ncurses.h>

#define LOGO_ROWS           6
#define LOGO_COLS           44
#define LOGO_TOP_MARGIN     1


extern WINDOW *title;
extern const char logo[LOGO_ROWS][LOGO_COLS + 1];

void setup_ui();
void refresh_ui();
void cleanup_ui();


#endif  //__H_UI_H__
