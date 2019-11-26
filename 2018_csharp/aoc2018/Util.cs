using System.IO;

namespace AoC
{
    public static class Util
    {
        public static string[] Input(int day)
        {
            return File.ReadAllLines( $"../../../Input/{day:00}.txt");
        }
    }
}