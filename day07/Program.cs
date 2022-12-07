using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Program
{
    public static void Main(string[] args)
    {
        string filepath = "input.txt";
        Part1(filepath);
        Part2(filepath);
    }

    class CustomDirectory
    {
        public string m_name;
        public CustomDirectory m_parent;
        public List<CustomDirectory> m_children = new List<CustomDirectory>();
        public List<CustomFile> m_files = new List<CustomFile>();
    }
    
    class CustomFile
    {
        public string m_name;
        public ulong m_size;
    }

    static CustomDirectory BuildFileSystem(IEnumerable<string> lines)
    {
        CustomDirectory cwd = null;
        CustomDirectory root = null;
        foreach (var line in lines)
        {
            var split = line.Split(' ');
            if (split[0] == "$")
            {
                if (split[1] == "cd")
                {
                    if (root == null)
                    {
                        root = new CustomDirectory()
                        {
                            m_name = split[2]
                        };
                        cwd = root;
                    }
                    else if (split[2] == "..")
                    {
                        cwd = cwd.m_parent;
                    }
                    else
                    {
                        var dir = cwd.m_children.Find(x => x.m_name.Equals(split[2]));
                        if (dir == null)
                        {
                            dir = new CustomDirectory()
                            {
                                m_parent = cwd,
                                m_name = split[2]
                            };
                            cwd.m_children.Add(dir);
                        }
                        cwd = dir;
                    }
                }
                else if (split[1] == "ls")
                {
                    // do nothing
                }
            }
            else if (split[0] == "dir")
            {
                var dir = cwd.m_children.Find(x => x.m_name.Equals(split[1]));
                if (dir == null)
                {
                    cwd.m_children.Add(new CustomDirectory()
                    {
                        m_parent = cwd,
                        m_name = split[1]
                    });    
                }
            }
            else if (ulong.TryParse(split[0], out ulong val))
            {
                var file = cwd.m_files.Find(x => x.m_name.Equals(split[1]));
                if (file == null)
                {
                    cwd.m_files.Add(new CustomFile()
                    {
                        m_name = split[1],
                        m_size = val
                    });    
                }
            }
        }

        return root;
    }
    
    static ulong CountSize(CustomDirectory dir, Dictionary<CustomDirectory, ulong> allDirs)
    {
        ulong size = 0;
        foreach (var file in dir.m_files)
        {
            size += file.m_size;
        }

        foreach (var child in dir.m_children)
        {
            size += CountSize(child, allDirs);
        }
        allDirs.Add(dir, size);
        return size;
    }
    
    static void Part1(string filepath)
    {
        var lines = File.ReadLines(filepath);
        var root = BuildFileSystem(lines);
        
        Dictionary<CustomDirectory, ulong> allDirs = new Dictionary<CustomDirectory, ulong>();
        ulong totalSize = CountSize(root, allDirs);
        Console.WriteLine($"Total Size {totalSize}");
        ulong sum = 0;
        foreach (var pair in allDirs)
        {
            if (pair.Value < 100_000)
            {
                sum += pair.Value;
            }
        }
        
        Console.WriteLine($"Part 1 = {sum}");
    }

    
    
    static void Part2(string filepath)
    {
        const ulong totalDiskSpace = 70_000_000;
        const ulong requiredUnusedSpace = 30_000_000;
        
        var lines = File.ReadLines(filepath);
        var root = BuildFileSystem(lines);
        
        Dictionary<CustomDirectory, ulong> allDirs = new Dictionary<CustomDirectory, ulong>();
        ulong totalSize = CountSize(root, allDirs);
        ulong requiredSpaceToClear = requiredUnusedSpace - (totalDiskSpace - totalSize);
        var sizes = allDirs.Values.ToArray();
        Array.Sort(sizes);
        ulong deletedFolderSize = Array.Find(sizes, x => x >= requiredSpaceToClear);
        Console.WriteLine($"Part 2 = {deletedFolderSize}");
    }
}
