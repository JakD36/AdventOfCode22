using System;
using System.IO;
using System.Collections.Generic;

class Program
{
    public static void Main()
    {
        Part1("input.txt"); 
        Part2("input.txt");        
    }

    public static void Part1(string filepath)
    {
        string text = File.ReadAllText(filepath);
        string[] elves = text.Split("\n\n");
        int[] caloriesPerElf = new int[elves.Length];
        for(int i = 0; i < elves.Length; ++i)
        {
            var calorieStrs = elves[i].Split("\n");
            foreach (var calorieStr in calorieStrs)
            {
                caloriesPerElf[i] += int.Parse(calorieStr);
            }
        }

        int indexOfMax = 0;
        int max = Int32.MinValue;
        for (int i = 0; i < caloriesPerElf.Length; ++i)
        {
            if (caloriesPerElf[i] > max)
            {
                max = caloriesPerElf[i];
                indexOfMax = i;
            }
        }
        
        Console.WriteLine($"Elf {indexOfMax + 1} has {max} calories");
    }
    
    public static void Part2(string filepath)
    {
        string text = File.ReadAllText(filepath);
        string[] elves = text.Split("\n\n");
        int[] caloriesPerElf = new int[elves.Length];
        for(int i = 0; i < elves.Length; ++i)
        {
            var calorieStrs = elves[i].Split("\n");
            foreach (var calorieStr in calorieStrs)
            {
                caloriesPerElf[i] += int.Parse(calorieStr);
            }
        }

        int[] max = new int[3];
        Array.Fill(max, int.MinValue);
        for (int i = 0; i < caloriesPerElf.Length; ++i)
        {
            for (int j = 0; j < max.Length; ++j)
            {
                if (caloriesPerElf[i] > max[j])
                {
                    int last = max[j];
                    for (int k = j+1; k < max.Length; ++k)
                    {
                        (last, max[k]) = (max[k], last);
                    }
                    max[j] = caloriesPerElf[i];
                    break;
                }
            }
        }

        int total = 0;
        for (int i = 0; i < max.Length; ++i)
        {
            total += max[i];
        }
        Console.WriteLine($"Total calories over top 3 {total}");
    }
}

