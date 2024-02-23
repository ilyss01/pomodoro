make:
	gcc -std=c17 -Wextra -Wall main.c -o pomodoro -lncurses

run:
	gcc -std=c17 -Wextra -Wall main.c -o pomodoro -lncurses; ./pomodoro

install:
	gcc -std=c17 -Wextra -Wall main.c -o pomodoro -lncurses; mv pomodoro /usr/local/bin/
