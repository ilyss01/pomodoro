#include <ncurses.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h> // for sleep

int main(int argc, char **argv) {
  if (argc > 1) {
    // TODO: implement taking args from CLI
  }

  // initialize empty screen
  initscr();

  // was taken from flappy bird in ncurses
  cbreak();
  noecho();

  // set cursor invisible
  curs_set(0);

  // initialize main window
  WINDOW *win = newwin(0, 0, 0, 0);
  refresh(); // without refresh it doesn't work at all, TODO: wrefresh

  // 0, 0 are for default border style
  box(win, 0, 0);

  // move cursor to middle of the screen
  // move(LINES/2, COLS/2);

  // print to screen
  // printw("hi");
  // alternative to move(...); printw(...); is mvprintw

  // print message at the center of the screen
  char msg[] = "hello";
  mvwprintw(win, (LINES / 2), ((COLS - strlen(msg) - 1) / 2), "hello");
  // show changes
  wrefresh(win);

  // refresh the screen, should not be used with win
  // refresh();

  // getting character quits the program
  getch();

  // printf("%i %i\n", LINES, COLS);
  endwin();
  delwin(win);

  // return terminal to it's state before the program
  echo();
  nocbreak();
  curs_set(1);
  clear();
}
