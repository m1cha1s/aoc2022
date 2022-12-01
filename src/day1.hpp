#pragma once

#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

#include "day.hpp"
#include "elf.hpp"

class Day1 final : public Day
{
private:
    std::ifstream file;

    std::string line;

    std::vector<Elf> elfs;

public:
    void run(std::string filename, bool dbg) override
    {
        if (filename == "")
            return;

        file.open(filename);

        while (std::getline(file, line))
        {
            std::stringstream s(line);
            int calories = 0;
            if (!(s >> calories))
            {
                elfs.push_back(Elf());
            }

            elfs.back().addCalories(calories);
        }

        int max = 0;
        for (auto elf : elfs)
        {
            if (dbg)
                std::cout << elf.getCalories() << std::endl;

            if (elf.getCalories() > max)
                max = elf.getCalories();
        }

        std::cout << "Max callories: " << max;
    }

    int getDay() override
    {
        return 1;
    }
};