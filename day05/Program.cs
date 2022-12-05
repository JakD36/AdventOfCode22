using System.IO;

class Day05
{
    public static void Main()
    {
        string filepath = "input.txt";
        // Part1(filepath);
        Part2(filepath);
    }

    static void Part1(string filepath)
    {
        string[] lines = File.ReadAllLines(filepath);
        int i = 0;
        int maxIdx = 0;
        List<List<(char id, int index)>> rows = new List<List<(char id, int index)>>();
        for (; i < lines.Length; ++i)
        {
            List<(char id, int index)> cols = new List<(char id, int index)>();
            int j = lines[i].IndexOf("[", StringComparison.Ordinal);
            if(j < 0)
                break;
            while (j >= 0)
            {
                maxIdx = Math.Max(j / 4, maxIdx);
                cols.Add((lines[i][j+1], j / 4)); // index of stack is (char index - 1)/4
                j = lines[i].IndexOf("[", j+1, StringComparison.Ordinal);
            }
            rows.Add(cols);
        }

        // Construct stacks
        Stack<char>[] stacks = new Stack<char>[maxIdx + 1];
        for (int j = 0; j < stacks.Length; ++j)
        {
            stacks[j] = new Stack<char>();
        }
        
        for (int j = rows.Count - 1; j >= 0; --j)
        {
            foreach (var col in rows[j])
            {
                stacks[col.index].Push(col.id);
            }
        }

        i += 2; // skip stack index line + white space

        for (; i < lines.Length; ++i)
        {
            string[] parts = lines[i].Split(" ");
            int count = int.Parse(parts[1]);
            int from = int.Parse(parts[3]) - 1;
            int to = int.Parse(parts[5]) - 1;

            for (int j = 0; j < count; ++j)
            {
                stacks[to].Push(stacks[from].Pop());
            }
        }

        for (int j = 0; j < stacks.Length; ++j)
        {
            Console.Write(stacks[j].Peek());
        }
    }
    
    static void Part2(string filepath)
    {
        string[] lines = File.ReadAllLines(filepath);
        int i = 0;
        int maxIdx = 0;
        List<List<(char id, int index)>> rows = new List<List<(char id, int index)>>();
        for (; i < lines.Length; ++i)
        {
            List<(char id, int index)> cols = new List<(char id, int index)>();
            int j = lines[i].IndexOf("[", StringComparison.Ordinal);
            if(j < 0)
                break;
            while (j >= 0)
            {
                maxIdx = Math.Max(j / 4, maxIdx);
                cols.Add((lines[i][j+1], j / 4)); // index of stack is (char index - 1)/4
                j = lines[i].IndexOf("[", j+1, StringComparison.Ordinal);
            }
            rows.Add(cols);
        }

        // Construct stacks
        List<char>[] stacks = new List<char>[maxIdx + 1];
        for (int j = 0; j < stacks.Length; ++j)
        {
            stacks[j] = new List<char>();
        }
        
        for (int j = rows.Count - 1; j >= 0; --j)
        {
            foreach (var col in rows[j])
            {
                stacks[col.index].Add(col.id);
            }
        }

        i += 2; // skip stack index line + white space

        for (; i < lines.Length; ++i)
        {
            string[] parts = lines[i].Split(" ");
            int count = int.Parse(parts[1]);
            int from = int.Parse(parts[3]) - 1;
            int to = int.Parse(parts[5]) - 1;

            stacks[to].AddRange(stacks[from].GetRange(stacks[from].Count - count, count));
            stacks[from].RemoveRange(stacks[from].Count - count, count);
        }

        for (int j = 0; j < stacks.Length; ++j)
        {
            Console.Write(stacks[j].Last());
        }
    }
}