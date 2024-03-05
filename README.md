# ORCA output parser

## Installation

`pip install orca-rs`

## Usage

```python
from orca_rs import parse_casscf, parse_tddft

output = parse_tddft("orca-tddft-results.out")

import pprint
pprint.pprint(output)
# {'soc_matrix': {'eigenvectors': [{'elements': [{'imag': 0.8511099815368652,
#                                                 'ms': 0,
#                                                 'real': -0.5249800086021423,
#                                                 'root': 0,
#                                                 'spin': 0,
#                                                 'weight': 0.9999899864196777}],
#                                   'energy': 0.0,
#                                   'n': 0},
#                                  {'elements': [{'imag': 0.5458400249481201,
#                                                 'ms': 0,
#                                                 'real': -0.07343000173568726,
#                                                 'root': 1,
#                                                 'spin': 1,
#                                                 'weight': 0.3033300042152405},
#                                                {'imag': 0.15578000247478485,
#                                                 'ms': -1,
#                                                 'real': 0.5692600011825562,
#                                                 'root': 1,
#                                                 'spin': 1,
#                                                 'weight': 0.34832999110221863},
#                                                {'imag': 0.00019999999494757503,
#                                                 'ms': 1,
#                                                 'real': 0.5901899933815002,
#                                                 'root': 1,
#                                                 'spin': 1,
#                                                 'weight': 0.34832999110221863}],
#                                   'energy': 24397.099609375,
#                                   'n': 1},
#                                   ...
#                                   ]
#                }
# }
```

