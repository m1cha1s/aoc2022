#pragma once

#include <iostream>

class Elf
{
private:
    int calories = 0;

public:
    Elf(){};

    void addCalories(int c)
    {
        calories += c;
    }

    int getCalories()
    {
        return calories;
    }
};