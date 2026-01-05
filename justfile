# https://just.systems

default: run

# build and run lers, generate and compile lers.yy.c, then run the binary file
run source="analyzer.l":
  @cargo run -- {{source}} 2>/dev/null
  @gcc lers.yy.c
  @./a.out
  @rm a.out lers.yy.c

# examples:

wc +FILES:
  @cargo run -- examples/wc.l 2>/dev/null
  @gcc lers.yy.c
  @./a.out {{FILES}}
  @rm a.out lers.yy.c

cat +FILES:
  @cargo run -- examples/cat.l 2>/dev/null
  @gcc lers.yy.c
  @./a.out {{FILES}}
  @rm a.out lers.yy.c

pascal +FILES="examples/pascal.pas":
  @cargo run -- examples/pascal.l 2>/dev/null
  @gcc lers.yy.c
  @./a.out {{FILES}}
  @rm a.out lers.yy.c
