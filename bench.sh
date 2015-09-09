#!/bin/bash -ex
g++ -O2 bench.cpp
time ./a.out

haxe -main Bench -cpp cpp
time cpp/Bench

javac Bench.java
time java -cp . Bench

haxe -main Bench -java java
time java -jar java/Bench.jar
