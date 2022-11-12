# points-to-grade

`points-to-grade` is a command-line-tool for generating grade distributions. 

## Usage

```shell
Usage: points-to-grade [OPTIONS] <POINTS>

Arguments:
  <POINTS>  

Options:
  -5, --half-points      use half points in grading
  -o, --output <OUTPUT>  Save output to file (Supported: .csv, .xlsx)
  -s, --scale <SCALE>    [default: german] [possible values: german, german-fos, english]
  -l, --linear           Uses the a linear grading algorithm instead of the IHK distribution
  -h, --help             Print help information
  -V, --version          Print version information
```

