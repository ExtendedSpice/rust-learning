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

var lines = File.ReadAllLines(options.Path);
foreach (var line in lines)
{
    if (line.Contains(options.Pattern))
        Console.WriteLine(line);
}

class GrrsOptions
{
    public string Pattern { get; set; } = string.Empty;
    public string Path { get; set; } = string.Empty;

    public override string ToString() => $"({Pattern}, {Path})";
}