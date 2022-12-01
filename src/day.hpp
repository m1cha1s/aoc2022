#pragma once

#include <string>

class Day
{
public:
    virtual void run(std::string, bool dbg) = 0;
    virtual int getDay() = 0;
};