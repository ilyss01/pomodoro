#include <ncurses.h> // graphics
#include <stdio.h>  // idk
#include <stdlib.h> // atoi
#include <string.h> // strlen
#include <unistd.h> // for sleep
#include <signal.h>

// TODO: switching between mode on pause
typedef enum state {
  PAUSE,
  WORK,
  BREAK,
} State;

// TODO: fix a bug with strange centering

// Length of given integer
unsigned int_len(int n) {
  unsigned counter = 0;
  while (n >= 1) {
    n /= 10;
    counter += 1;
  }
  return counter;
}

// Renders box around the screen and timer in the middle
void render_timer(WINDOW *win, unsigned *time) {
  uint8_t formated_mins = *time / 60;
  uint8_t formated_secs = *time % 60;
  uint8_t y_pos = LINES / 2;
  uint8_t x_pos;
  wclear(win);
  box(win, 0, 0);
  if (formated_secs < 10) {
    // Additional zero is added in timer, so it must be subtracted from x
    // coord
    x_pos = (COLS - int_len(formated_mins) - int_len(formated_secs) - 1) / 2;
    mvwprintw(win, y_pos, x_pos, "%i:0%i", formated_mins, formated_secs);
  } else {
    x_pos = (COLS - int_len(formated_mins) - int_len(formated_secs)) / 2;
    mvwprintw(win, y_pos, x_pos, "%i:%i", formated_mins, formated_secs);
  }
  // Show changes
  wrefresh(win);
}

// Renders box around the screen and given word in the middle
void render_word(WINDOW *win, const char *word) {
  uint8_t y_pos = LINES / 2;
  uint8_t x_pos = (COLS - strlen("Pause"))/2;
  wclear(win);
  box(win, 0, 0);
  mvwprintw(win, y_pos, x_pos, "Pause");
  wrefresh(win);
}

// Handling pause screen
void pause_screen(WINDOW *win) {
  char input_char;
  while (true) {
    render_word(win, "Pause");
    input_char = getch();
    if (input_char == ' ') {
      return;
    }
  }
}

// Renders the screen, counts the timer, sets pause
void run_timer(WINDOW *win, unsigned time) {
  uint8_t formated_mins;
  uint8_t formated_secs;
  char input_char;

  while (time != 0) {
    formated_mins = time / 60;
    formated_secs = time % 60;

    render_timer(win, &time);
    time -= 1;

    // Getting space quits the program
    // getch is set to halfdelay(10) so every 10ms it gets ERR if nothing is
    // pressed
    input_char = getch();
    if (input_char == ERR) {
      continue;
    } else if (input_char == 27 || input_char == 'q') {
      // <ESC> or q should stop the timer
      // TODO: stop the timer
    } else if (input_char == ' ') {
      pause_screen(win);
    }
  }
}

// Transition screen between modes
void transition_screen(WINDOW *win) {
    char input_char;
    while (true) {
		render_word(win, "Press any key");
		input_char = getch();
		if (input_char == ERR) {
    		continue;
		} else {
    		return;
		}
    }
}

int main(int argc, char **argv) {
  // Time set
  unsigned work_time = 25 * 60;
  unsigned short_break_time = 5 * 60;
  unsigned long_break_time = 20 * 60;
  unsigned cycles_lim = 4;
  unsigned cycle = 0;
  State state = WORK;

  // Getting args via CLI, like: "pomodoro 25 5 0"
  // TODO: unsafe, check to atoi
  if (argc == 5) {
    work_time = (unsigned)(atoi(argv[1]) * 60);
    short_break_time = (unsigned)(atoi(argv[2]) * 60);
    cycles_lim = (unsigned)(atoi(argv[3]));
    long_break_time = (unsigned)(atoi(argv[4]));
  }

  // Initialize empty screen
  initscr();
  if (stdscr == NULL) {
    perror("Couldn't Initialize a screen");
    exit(1);
  }

  // Was taken from flappy bird in ncurses
  // cbreak();
  noecho();
  halfdelay(10);
  
  curs_set(0); // Set cursor invisible

  // Initialize main window full screen
  WINDOW *win = newwin(0, 0, 0, 0);

  // signal(SIGINT, );

  while (true) {
    run_timer(win, work_time);
    transition_screen(win);
    state = BREAK;
    cycle += 1;
    if (cycle == cycles_lim) {
      run_timer(win, long_break_time);
      cycle = 0;
    } else {
      run_timer(win, short_break_time);
    }
    transition_screen(win);
  }

  // Without refresh it doesn't work at all, still don't know why
  // TODO: wrefresh
  refresh();
  // Cleaning procedure
  clear();
  endwin();
  delwin(win);

  // Return terminal to it's state before the program
  echo();
  nocbreak();
  curs_set(1);
  // execl("stty", "sane");
}
