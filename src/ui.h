#ifndef __H_UI_H__
#define __H_UI_H__

#include <ncurses.h>

#define LOGO_BORDER_ENABLED             1
#define LOGO_TOP_MARGIN                 1
#define LOGO_PADDING                    1       // Only relevant if using border; Is simply additional margin otherwise

#define LOGO_ROWS                       6       // No. of strings in the logo array
#define LOGO_COLS                       44      // Max. string length from logo array

#define MAIN_CONTENT_BORDER_ENABLED     1       // FIXME: Arbitrary values for now
#define MAIN_CONTENT_MARGIN             1
#define MAIN_CONTENT_PADDING            1

#define MAIN_CONTENT_ROWS               5
#define MAIN_CONTENT_COLS               52


/*
 * The calculations below determine the dimensions of the window
 * in which the logo is contained.
 *
 * (LOGO_BORDER_ENABLED*2) -> Total no. columns for the border.
 * (LOGO_PADDING*2)        -> Total no. columns for the padding.
 */
#define LOGO_WIN_ROWS       (LOGO_ROWS + (LOGO_BORDER_ENABLED*2) + (LOGO_PADDING*2))
#define LOGO_WIN_COLS       (LOGO_COLS + (LOGO_BORDER_ENABLED*2) + (LOGO_PADDING*2))


extern WINDOW *logo_win;
extern const char logo[LOGO_ROWS][LOGO_COLS + 1];

void setup_ui();
void refresh_ui();
void cleanup_ui();
void display_resize_warning(int rows, int cols);


#endif  //__H_UI_H__
