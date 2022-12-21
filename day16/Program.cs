using System.Text.RegularExpressions;
using System.Diagnostics;

string filepath = "input.txt";
Part1(filepath);


void Part1(string filepath)
{
    var sw = Stopwatch.StartNew();
    var valves = BuildDataStructure(filepath);
    Console.WriteLine($"Building graphs took {sw.ElapsedMilliseconds} ms");
    sw.Restart();
    int max = CalculateMax("AA", valves, new HashSet<string>() { "AA" }, 30, 0);
    Console.WriteLine($"Calculating max took {sw.ElapsedMilliseconds} ms");
    Console.WriteLine($"Part 1 {max}");
}

int CalculateMax(string current, Dictionary<string, Valve> valves, HashSet<string> set, int timeRemaining, int score)
{
    int max = score;
    foreach (var (name, distance) in valves[current].m_tunnelsTo)
    {
        if(set.Contains(name))
            continue;
        var cp = new HashSet<string>(set);
        cp.Add(name);
        
        int remainingTime = timeRemaining - (distance + 1);
        int val = 0; 
        if(remainingTime >= 0)
        {
            val = CalculateMax(name, valves, cp, remainingTime, score + remainingTime * valves[name].m_flowRate);    
        }
        max = Math.Max(val, max);
    }
    return max;
}

Dictionary<string, Valve> BuildDataStructure(string filepath)
{
    string[] lines = File.ReadAllLines(filepath);
    var re = new Regex(@"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (?:([A-Z]{2})(?:, )?)+");

    Dictionary<string, Valve> valves = new Dictionary<string, Valve>();
    
    foreach (var line in lines)
    {
        var match = re.Match(line);
        GroupCollection groups = match.Groups;
        var name = groups[1].Value;

        Valve valve = new Valve
        {
            m_flowRate = int.Parse(groups[2].Value),
            m_tunnelsTo = new Dictionary<string, int>()
        };
        foreach (var capture in groups[3].Captures)
        {
            valve.m_tunnelsTo.Add(capture.ToString(), 1);
        }
        valves.Add(name, valve);
    }

    var final = new Dictionary<string, Valve>();
    foreach (var pair in valves)
    {
        final.Add(pair.Key, new Valve()
        {
            m_flowRate   = pair.Value.m_flowRate,
            m_tunnelsTo = new Dictionary<string, int>(pair.Value.m_tunnelsTo)
        });
    }

    foreach (var (key, value) in valves)
    {
        Queue<(string, int)> toCheck = new Queue<(string, int)>();
        foreach (var leadTo in value.m_tunnelsTo.Keys)
        {
            toCheck.Enqueue((leadTo, 1));
        }

        while (toCheck.Count > 0)
        {
            var (leadTo, dist) = toCheck.Dequeue();
            foreach (var next in valves[leadTo].m_tunnelsTo.Keys)
            {
                if(next == key)
                    continue;
                
                if (final[key].m_tunnelsTo.ContainsKey(next) == false)
                {
                    final[key].m_tunnelsTo.Add(next, dist + 1);
                    toCheck.Enqueue((next, dist + 1));    
                }
                else if (final[key].m_tunnelsTo[next] > dist + 1)
                {
                    final[key].m_tunnelsTo[next] = dist + 1;
                    toCheck.Enqueue((next, dist + 1));
                }
            }   
        }
    }

    foreach (var (name, valve) in valves)
    {
        if (valve.m_flowRate == 0 && name != "AA")
        {
            foreach (var graphNode in final.Values)
            {
                graphNode.m_tunnelsTo.Remove(name);
            }
        }
    }

    return final;
}

struct Valve
{
    public int m_flowRate;
    public Dictionary<string, int> m_tunnelsTo;
};