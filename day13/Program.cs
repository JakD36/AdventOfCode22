using System;
using System.Collections.Generic;
using System.IO;

public class Packet
{}
    
public class PacketList : Packet
{
    public List<Packet> m_packets = new List<Packet>();
}
    
public class PacketInt : Packet
{
    public int m_value;
}

static class Program
{
    public static void Main()
    {
        string filepath = "input.txt";
        Part1(filepath);
        Part2(filepath);
    }
    
    public static void Part1(string filepath)
    {
        var lines = File.ReadAllText(filepath).Split(new []{"\n\n"}, StringSplitOptions.RemoveEmptyEntries);
        var pairs = Array.ConvertAll(lines, input => input.Split('\n'));
        int sum = 0;
        var comparer = new PacketListComparer();
        for(int i = 0; i < pairs.Length; ++i)
        {
            var first = BuildList(pairs[i][0]);
            var second = BuildList(pairs[i][1]);

            if (comparer.Compare(first, second) <= 0)
            {
                sum += i + 1;
            }
        }
        Console.WriteLine($"Part 1 {sum}");
    }
    
    public static void Part2(string filepath)
    {
        List<PacketList> packets = new List<PacketList>();
        foreach (var line in File.ReadLines(filepath))
        {
            if(string.IsNullOrWhiteSpace(line) == false)
                packets.Add(BuildList(line));
        }
        
        PacketList start = BuildList("[[2]]");
        PacketList end = BuildList("[[6]]");
        packets.Add(start);
        packets.Add(end);
        packets.Sort(new PacketListComparer());

        Console.WriteLine($"Part 2 {(packets.IndexOf(start) + 1) * (packets.IndexOf(end) + 1)}");
    }

    
    public static int ComparePackets(Packet a, Packet b)
    {
        // Recursive patterns apparently a C# 8.0 feature
        return (a, b) switch 
        {
            {a: PacketInt aInt, b: PacketInt bInt} => aInt.m_value.CompareTo(bInt.m_value),
            {a: PacketInt aInt, b: PacketList bList} => new PacketListComparer().Compare(new PacketList(){m_packets = new List<Packet>(){aInt}}, bList),
            {a: PacketList aList, b: PacketInt bInt} => new PacketListComparer().Compare(aList, new PacketList(){m_packets = new List<Packet>(){bInt}}),
            {a: PacketList aList, b: PacketList bList} => new PacketListComparer().Compare(aList, bList),
        };
    }
    

    public class PacketListComparer : IComparer<PacketList>
    {
        public int Compare(PacketList a, PacketList b)
        {
            var len = Math.Min(a.m_packets.Count, b.m_packets.Count);

            for (int i = 0; i < len; ++i)
            {
                switch (ComparePackets(a.m_packets[i], b.m_packets[i]))
                {
                    case 1:
                        return 1;
                    case -1:
                        return -1;
                    case 0:
                        break;
                }
            }

            return Math.Sign(a.m_packets.Count - b.m_packets.Count);
        }       
    }

    public static PacketList BuildList(string line)
    {
        var root = new PacketList();
        Stack<PacketList> stack = new Stack<PacketList>();
        stack.Push(root);

        for (int i = 1; i < line.Length;)
        {
            switch (line[i])
            {
                case '[': 
                    stack.Push(new PacketList());
                    ++i;
                    break;
                case ']':
                    var lastPackets = stack.Pop();
                    if (stack.Count > 0)
                    {
                        stack.Peek().m_packets.Add(lastPackets);
                    }
                    ++i;
                    break;
                case ',':
                    ++i;
                    break;
                default:
                    int nextComma = line.IndexOf(",", i, StringComparison.Ordinal);
                    int nextListEnd = line.IndexOf("]", i, StringComparison.Ordinal);
                    int skip = -1;
                    if (nextComma == -1)
                    {
                        skip = nextListEnd;
                    }
                    else if (nextListEnd == -1)
                    {
                        skip = nextComma;
                    }
                    else if (nextComma < nextListEnd)
                    {
                        skip = nextComma;
                    }
                    else if (nextListEnd < nextComma)
                    {
                        skip = nextListEnd;
                    }
                    
                    int value = int.Parse(line.Substring(i, skip - i));
                    stack.Peek().m_packets.Add(new PacketInt(){m_value = value});
                    i += skip - i; 
                    break;
            }
        }
        return root;
    }
}
