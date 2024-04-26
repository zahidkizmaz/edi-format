# EDIFACT Formatter

Simple EDI file formatter.

### Usage

```
edi-format [OPTIONS] [FILE_NAME]

Arguments:
  [FILE_NAME]  File path to format [default: .]

Options:
  -d, --dry-run  Do not actually modify the file but show formatted content in stdout
  -h, --help     Print help
  -V, --version  Print version
```

## Example EDIFACT File:

```
UNA:+.? '
UNB+IATB:1+6XPPC:ZZ+LHPPC:ZZ+940101:0950+1'
UNH+1+PAORES:93:1:IA'
MSG+1:45'
IFT+3+XYZCOMPANY AVAILABILITY'
ERC+A7V:1:AMD'
IFT+3+NO MORE FLIGHTS'
ODI'
TVL+240493:1000::1220+FRA+JFK+DL+400+C'
PDI++C:3+Y::3+F::1'
APD+74C:0:::6++++++6X'
TVL+240493:1740::2030+JFK+MIA+DL+081+C'
PDI++C:4'
APD+EM2:0:1630::6+++++++DA'
UNT+13+1'
UNZ+1+1'
```

##### Reference

https://en.wikipedia.org/wiki/EDIFACT
