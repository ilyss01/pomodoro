# CLI pomodoro timer

Yet another pomodoro timer. Right now it just works although I see another way of improving it. I'll take a while.

Examples of usage:

`pomodoro` should run just fine without any arguments. Default behaviout is:
* 25 minutes of work
* 5 minutes of short break
* 15 minutes of long break
* 4 intervals of short breaks before long one
```
pomodoro
```

If you'd like to change that time, you can do that in two ways:

```
pomodoro 25 5 15 4
```
Given paramethers correspond to work, short break, long break and intervals 

Also explicit params version is aviable
```
pomodoro --work 25 --short 5 --long 15 --cycles 4
```
