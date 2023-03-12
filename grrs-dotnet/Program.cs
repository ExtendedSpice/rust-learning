Console.WriteLine("Hello, World!");

Console.WriteLine(string.Join(" ", args));

var pattern = args.ElementAtOrDefault(1) ?? throw new ArgumentException("pattern is missing");
var path = args.ElementAtOrDefault(2) ?? throw new ArgumentException("path is missing");

Console.WriteLine(pattern);
Console.WriteLine(path);

var options = new GrrsOptions
{
    Pattern = pattern,
    Path = path
};
Console.WriteLine(options);

try
{
    using var reader = new StreamReader(options.Path);
    while (reader.ReadLine() is string line)
    {
        if (line.Contains(options.Pattern))
            Console.WriteLine(line);
    }
}
catch (Exception e)
{
    Console.WriteLine($"error on reading file: {e}");
    return;
}


class GrrsOptions
{
    public string Pattern { get; set; } = string.Empty;
    public string Path { get; set; } = string.Empty;

    public override string ToString() => $"({Pattern}, {Path})";
}