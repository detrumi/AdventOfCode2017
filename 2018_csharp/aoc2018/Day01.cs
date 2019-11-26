using System;
using System.Collections.Generic;

namespace AoC
{
    public class Day01
    {
        public void Answer()
        {
            var found = new HashSet<int>();
            var n = 0;
            for (var i = 0; ; i++)
            {
                foreach (var line in Util.Input(1))
                {
                    n += int.Parse(line);
                    if (found.Contains(n))
                    {
                        Console.WriteLine($"Part 2: {n}");
                        return;
                    }

                    found.Add(n);
                }

                if (i == 0) Console.WriteLine($"Part 1: {n}");
            }
        }
    }
}