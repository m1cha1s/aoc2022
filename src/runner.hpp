#pragma once

#include <vector>
#include <utility>
#include <memory>
#include <string>
#include <iostream>

#include "day.hpp"

class Runner
{
private:
    std::vector<std::pair<std::shared_ptr<Day>, std::pair<std::string, std::string>>> days;

public:
    Runner() {}
    ~Runner() {}

    void addDay(Day *day, std::string data, std::string exampleData, bool dbg = true)
    {
        days.push_back(std::pair(std::shared_ptr<Day>(day), std::pair(data, exampleData)));
    }

    void run(bool dbg = true)
    {
        for (auto day : days)
        {
            std::cout << "---- Day " << day.first->getDay() << "----\n";

            day.first->run(day.second.first, dbg);
        }
    }

    void run(int day, bool dbg = true)
    {
        std::cout << "---- Day " << day << "----\n";
        days[day - 1].first->run(days[day - 1].second.first, dbg);
    }

    void runExample(bool dbg = true)
    {
        for (auto day : days)
        {
            std::cout << "---- Day " << day.first->getDay() << " ----\n";

            day.first->run(day.second.second, dbg);
        }
    }

    void runExample(int day, bool dbg = true)
    {
        std::cout << "---- Day " << day << " ----\n";
        days[day - 1].first->run(days[day - 1].second.second, dbg);
    }
};