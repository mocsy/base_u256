# base-u256 encode/decode
base-u256 is to utf-8 as base-64 is to ascii

base64 takes 3 bytes at a time and then converts them to 4 bytes each with 64 possibilities.
|bits|possibilities|
|:---|:---|
|2^24 | 16777216|
|64^4 | 16777216|

base-u256 takes 3 bytes at a time and then converts them to 3 utf-8 2 byte characters each with 256 possibilities.

The encoded bytes is exactly two times larger than the original binary.

|bits|possibilities|
|:---|:---|
|2^24 | 16777216|
|256^3 | 16777216|

# base-u1024 encode/decode
base-u1024 takes 30 bits at a time and then converts them to 3 utf-8 2 byte characters each with 1024 possibilities.

The encoded bytes is 1.5 times larger than the original binary.

|bits|possibilities|
|:---|:---|
|2^30 | 1073741824|
|1024^3 | 1073741824|
