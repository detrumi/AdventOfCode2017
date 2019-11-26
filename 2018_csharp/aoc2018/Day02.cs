using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace AoC
{
    public class Day02
    {
        public void Answer()
        {
            Part1();
            Part2();
        }

        void Part1()
        {
            var (twos, threes) = (0, 0);
            foreach (var line in Util.Input(2))
            {
                var two = false;
                var three = false;
                for (var i = 0; i < 26; i++)
                {
                    var letter = (char)('a' + i);
                    var count = line.Count(c => c == letter);
                    if (count == 2) two = true;
                    if (count == 3) three = true;
                }

                if (two) twos++;
                if (three) threes++;
            }
            Console.WriteLine($"Part 1: {twos * threes}");
        }

        void Part2()
        {
            var equalities = Enumerable.Repeat(new HashSet<string>(), 26).ToArray();
            foreach (var line in Util.Input(2))
            {
                for (var i = 0; i < line.Length; i++)
                {
                    var s = new StringBuilder(line) {[i] = '_'}.ToString();
                    if (equalities[i].Contains(s))
                    {
                        Console.WriteLine($"Part 2: {s.Replace("_", "")}");
                    }

                    equalities[i].Add(s);
                }
            }
        }
    }
}