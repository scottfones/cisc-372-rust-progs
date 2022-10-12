# Breakout 05: Hi10

1. Write a C/MPI program in which each process does the following:

  print "i: hi\n" 10 times, where i is the rank.  fflush after each print.
  print "i: bye\n" 10 times, fflushing after each print.

Call the program hi10.c.  Compile and run the program with different
process counts.  Run it with 2 procs over and over again.  Do you ever
see anything strange?

-----

2. Modify hi10.c so that all the hi lines (from all processes) are
printed before any of the bye lines are printed. What function do you
need?

-----

