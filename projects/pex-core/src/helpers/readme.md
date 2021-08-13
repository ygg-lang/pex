| Input     | Output            |
|-----------|-------------------|
| #A        | (A, A, A, 255)    |
| #AB       | (AB, AB, AB, 255) |
| #ABC      | (A, B, C, 255)    |
| #ABCD     | (AA, BB, CC, DD)  |
| =5        | Error             |
| #ABCDEF   | (AB, CD, EF, 255) |
| =7        | Error             |
| #ABCDEFGH | (AB, CD, EF, GH)  |
| >8        | Error             |