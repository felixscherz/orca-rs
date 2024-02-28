
## API

start with filename as input,
return a dict with the different sections as keys


while reading a file, read until the start of a particular section is identified
once identified, pass the buffer to an underlying parser that reads its section
once complete, return to the caller

## Strategy

The part of the log file that we want to parse looks like this:

```
The threshold for printing is 0.0100
Eigenvectors of the SOC matrix:

             E(cm-1)  Weight      Real         Imag     : Root  Spin  Ms
 STATE  0:      0.00
                      0.99999     -0.52498      0.85111 : 0     0     0
 STATE  1:  24397.10
                      0.30333     -0.07343      0.54584 : 1     1     0
                      0.34833      0.56926      0.15578 : 1     1    -1
                      0.34833      0.59019      0.00020 : 1     1     1
 STATE  2:  24397.11
                      0.69666     -0.11130      0.82721 : 1     1     0
                      0.15167     -0.37563     -0.10279 : 1     1    -1
                      0.15167     -0.38944     -0.00014 : 1     1     1
 STATE  3:  24397.32
                      0.50000     -0.42206     -0.56733 : 1     1    -1
                      0.50000      0.55669      0.43599 : 1     1     1
 STATE  4:  31302.57
                      0.99997      0.13305     -0.99110 : 1     0     0
 STATE  5:  40139.95
                      0.69647     -0.16888      0.81728 : 2     1     0
                      0.15176     -0.36760     -0.12896 : 2     1    -1
                      0.15176     -0.38861     -0.02731 : 2     1     1
 STATE  6:  40139.97
                      0.30352      0.09951     -0.54187 : 2     1     0
                      0.34824     -0.56103     -0.18296 : 2     1    -1
                      0.34824     -0.58943     -0.02832 : 2     1     1
 STATE  7:  40140.04
                      0.49999      0.68740      0.16573 : 2     1    -1
                      0.49999     -0.70677      0.02168 : 2     1     1
 STATE  8:  60781.67
                      0.30384      0.53968      0.11221 : 3     1     0
                      0.34808      0.19586     -0.55652 : 3     1    -1
                      0.34808      0.04220     -0.58847 : 3     1     1
 STATE  9:  60781.73
                      0.50000      0.18852     -0.68151 : 3     1    -1
                      0.50000     -0.00107      0.70710 : 3     1     1
 STATE 10:  60781.73
                      0.69615      0.82718      0.10921 : 3     1     0
                      0.15192     -0.10293      0.37594 : 3     1    -1
                      0.15192      0.00183      0.38977 : 3     1     1
 STATE 11:  63711.22
                      0.50000     -0.18851      0.68152 : 4     1    -1
                      0.50000      0.00017     -0.70711 : 4     1     1
 STATE 12:  63711.22
                      0.29304      0.53645      0.07259 : 4     1     0
                      0.35348      0.15832     -0.57307 : 4     1    -1
                      0.35348     -0.00032     -0.59454 : 4     1     1
 STATE 13:  63711.22
                      0.70696     -0.83717     -0.07817 : 4     1     0
                      0.14652      0.08628     -0.37293 : 4     1    -1
                      0.14652     -0.01575     -0.38246 : 4     1     1
 STATE 14:  69145.73
                      1.00000      0.99271      0.12050 : 2     0     0
 STATE 15:  71921.90
                      1.00000     -0.99141     -0.13081 : 3     0     0
 STATE 16:  75878.64
                      0.69770      0.11150     -0.82781 : 5     1     0
                      0.15115      0.37492      0.10287 : 5     1    -1
                      0.15115      0.38878      0.00000 : 5     1     1
 STATE 17:  75878.64
                      0.29901      0.13991     -0.52862 : 5     1     0
                      0.35049     -0.51452     -0.29286 : 5     1    -1
                      0.35050     -0.59203      0.00000 : 5     1     1
 STATE 18:  75878.64
                      0.49836     -0.63797     -0.30225 : 5     1    -1
                      0.49836      0.70594      0.00000 : 5     1     1
 STATE 19:  77467.36
                      1.00000     -0.19972      0.97985 : 4     0     0
 STATE 20:  84397.49
                      1.00000      0.13145     -0.99132 : 5     0     0

-------------------------
TD-DFT-EXCITATION SPECTRA
```


1. Identify the start of the eigenvectors block
2. inside the block, identify lines into enums -> header, state, component, end-of-block
3. this turns the lines into tokens -> next step is assemble tokens into actual vectors/higher-level objetcs