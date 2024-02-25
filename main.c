#include <ncurses.h> // graphics
#include <stdio.h>   // idk
#include <stdlib.h>  // atoi
#include <string.h>  // strlen
#include <unistd.h>  // for sleep

// The program kind of basically works
// TODO: split in functions, make it circle by work->time->work->time...
// TODO: put initialization of time outside of loop

// For centering timer on screen
#define COLON_LENGTH 1

// https://stackoverflow.com/questions/9208296/ncurses-keyboard-input

// Count length of given integer by counting it's digits
int int_len(int n) {
  int counter = 0;
  while (n >= 1) {
    n = n / 10;
    counter++;
  }
  return counter;
}

int main(int argc, char **argv) {
  // Basic time set
  int work_time = 25 * 60;
  int break_time = 5 * 60;
  uint8_t cycles = 0;
  char input_char;

  // Getting args via CLI, like: "pomodoro 25 5 0"
  // TODO: unsafe, check to atoi
  if (argc == 4) {
    work_time = atoi(argv[1]) * 60;
    break_time = atoi(argv[2]) * 60;
    cycles = atoi(argv[3]);
  }

  // Initialize empty screen
  // TODO: unsafe, place checks
  initscr();

  // Was taken from flappy bird in ncurses
  //cbreak();
  noecho();
  halfdelay(10);

  // Set cursor invisible
  curs_set(0);

  // Initialize main window full screen
  WINDOW *win = newwin(0, 0, 0, 0);
  // Without refresh it doesn't work at all, still don't know why
  // TODO: wrefresh
  refresh();

  while (true) {
    wclear(win);
    // 0, 0 are for default border style
    box(win, 0, 0);

    uint8_t formated_mins = work_time / 60;
    uint8_t formated_secs = work_time % 60;
    uint8_t y_pos = LINES / 2;
    // uint8_t x_pos = (COLS / 2) - int_len(formated_mins) - COLON_LENGTH +
                    // int_len(formated_secs);
    uint8_t x_pos = (COLS - int_len(formated_mins) - int_len(formated_secs))/2;

    // Print message at the center of the screen
    if (formated_secs < 10) {
	  mvwprintw(win, y_pos, x_pos, "%i:0%i", formated_mins, formated_secs);
    } else {
      mvwprintw(win, y_pos, x_pos, "%i:%i", formated_mins, formated_secs);
    }

    // Show changes
    wrefresh(win);
    // TODO: sleep -> napms
    // https://stackoverflow.com/questions/72111820/how-can-i-use-napms-in-ncurses
    // sleep(1);
    work_time -= 1;

    // Getting space quits the program
    // getch is set to halfdelay(10) so every 10ms it gets ERR if nothing is pressed
    input_char = getch();
    if (input_char == ERR) {
      continue;
    } else if (input_char == ' ') {
      break;
    }
  }

  // Cleaning procedure
  endwin();
  delwin(win);

  // Return terminal to it's state before the program
  echo();
  nocbreak();
  curs_set(1);
  clear();
}
