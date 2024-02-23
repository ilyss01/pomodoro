#include <stdio.h>
#include <unistd.h> // for sleep
#include <ncurses.h>

int main() {
    // initialize empty screen
    initscr();

	// was taken from flappy bird in ncurses
    cbreak();
    noecho();

    WINDOW *win = newwin(LINES , COLS, 0, 0);

	// 0, 0 are for default border style
    box(win, 0, 0);
    wrefresh(win);

	// set cursor invisible
    curs_set(0);

	// move cursor to middle of the screen
    //move(LINES/2, COLS/2);

	// print to screen
	//printw("hi");
	// alternative to move(...); printw(...); is mvprintw

	//mvwprintw(win, LINES/2, COLS/2, "hello");

	// refresh the screen, should not be used with win
	//refresh();

    // getting character quits the program
    getch();

    // printf("%i %i\n", LINES, COLS);
    endwin();
}
