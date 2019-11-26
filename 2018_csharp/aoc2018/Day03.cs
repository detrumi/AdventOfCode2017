using System;
using System.Collections.Generic;
using System.Linq;

namespace AoC
{
    struct Pos
    {
        public int X { get; set; }
        public int Y { get; set; }
    }

    public class Day03
    {
        public void Answer()
        {
            var tiles = new Dictionary<Pos, List<int>>();
            foreach (var line in Util.Input(3))
            {
                var parts = line.Split();
                var id = int.Parse(parts[0].Substring(1));
                var margins = parts[2]
                    .Substring(0, parts[2].Length - 1)
                    .Split(',')
                    .Select(int.Parse)
                    .ToArray();
                var sizes = parts[3]
                    .Split('x')
                    .Select(int.Parse)
                    .ToArray();
                for (var x = margins[0]; x < margins[0] + sizes[0]; x++)
                {
                    for (var y = margins[1]; y < margins[1] + sizes[1]; y++)
                    {
                        var pos = new Pos() {X = x, Y = y};
                        if (tiles.ContainsKey(pos))
                        {
                            tiles[pos].Add(id);
                        }
                        else
                        {
                            tiles.Add(pos, new List<int>() { id });
                        }
                    }
                }
            }
            Console.WriteLine($"Part 1: {tiles.Values.Count(v => v.Count > 1)}");

            var keys = tiles.Values.SelectMany(v => v).ToHashSet();
            foreach (var value in tiles.Values)
            {
                if (value.Count > 1)
                {
                    keys.RemoveWhere(k => value.Contains(k));
                }
            }
            Console.WriteLine($"Part 2: {keys.Min()}");
        }
    }
}