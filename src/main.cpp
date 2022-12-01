#include <iostream>
#include <fstream>

#include "runner.hpp"

#include "day1.hpp"

int main()
{
  // std::ifstream day1Data("../data/day1Example.txt");

  Runner runner;

  runner.addDay(new Day1, "../data/day1.txt", "../data/day1Example.txt");

  runner.run();
}