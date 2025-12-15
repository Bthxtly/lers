# https://just.systems

# build and run lers, generate and compile lers.yy.c, then run the binary file
run:
  @cargo run
  @gcc lers.yy.c
  @./a.out

default: run
