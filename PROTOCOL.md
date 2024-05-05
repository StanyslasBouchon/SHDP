# SHDP Binary Explanation
## Summary


## Introduction
SHDP is a binary protocol that allows sending and receiving data in a compressed way. It is especially useful when sending files. **The protocol is not standardized yet and is still in development**.

## Data types
* `u8`: unsigned 8 bits (1 byte)
* `u16`: unsigned 16 bits (2 bytes)
* `u32`: unsigned 32 bits (4 bytes)
* `NuL`: number of unsigned type. `8u8` is equivalent to `u8 u8 u8 u8 u8 u8 u8 u8`
* `Fy`: unsigned fyve (5 bits)
* `[x]`: array of `x` type. Example: `[u8]` is an array of unsigned bytes.

## Basic frame
The basic frame of SHDP is as follows:
* `u8`: the SHDP version
* `u16`: the SHDP event code
* `u32`: the SHDP data length in bit (max $2^{32} - 1$, min $8$)
* `[u8]`: the SHDP data

### Example
Let's take an example with the following data:
* SHDP version: $1$
* SHDP event code: $1$
* SHDP data length: $13$
* SHDP data: `Hello, World!` (ASCII)

The binary representation of the frame will be:
```plaintext
00000000 01 00 01 00 00 00 68 48 65 6C 6C 6F 2C 20 57 6F ......hHello, Wo
00000010 72 6C 64 21                                     rld!
```


## Event codes
Event codes are used to determine the type of data that is sent and how the data is structured. The event code is a 16-bit unsigned integer.

* `0x0000`: `COMPONENT_NEEDS_REQUEST`
* `0x0001`: `HTML_FILE_RESPONSE`
* `0x0002`: `ERROR_RESPONSE`
* `0x0003`: `COMPONENT_NEEDS_RESPONSE`
* `0x0004`: `FULL_FYVE_RESPONSE`
* `0x0005`: `INTERACTION_REQUEST`
* `0x0006`: `INTERACTION_RESPONSE`

> [!NOTE]
> Other event codes are reserved for future use, but every event code above `0x1000` is reserved for private use, which means that users can use them as they want.

## Circumstances
> [!IMPORTANT]
> The following rules are mandatory for both the client and the server.

An SHDP server MUST NOT send a frame with a data length of $0$, and the user MUST NOT accept it.<br>
An SHDP client MUST NOT send a frame with a data length of $0$, and the server MUST answer with an `ERROR_RESPONSE` frame with the the appropriate error code.<br>
An SHDP server MUST NOT send a framle without answering a request from a client, and the client MUST NOT accept it.

## Events
### COMPONENT_NEEDS_REQUEST
This event is sent by a client to a server to request a component. The server MUST answer with a `COMPONENT_NEEDS_RESPONSE` event followed by `HTML_FILE_RESPONSE` and/or `FULL_FYVE_RESPONSE` events.

The frame data contains the component name to the end of the frame.

### HTML_FILE_RESPONSE
This event is sent by a server to a client to send an HTML file after receiving a `COMPONENT_NEEDS_REQUEST`. The frame data contains the HTML file encoded in fyves.

#### Data structure
* `[u8]`: The file name
* `u8=0`: Basic separator
* `[Fy & u8]`: The compressed HTML file

> [!NOTE]
> Because the data could have a length not divisible by $8$, the last byte will be padded with zeros. These leading zeros are not counted in the data length.

#### Operating codes
Operating codes are fyve-based data that are used to know how to interpret the data. The operating codes are as follows:

They are all starting with `00000`, which means that `00000` implies operations next to them.

> [!CAUTION]
> No fyve data is `00000` nor `11111`. Using these codes to encode data will result in bad interpretation or behavior.

* `00000 00000` (`0x00`): start of UTF-8 chain
* `00000 10000` (`0x10`): start of tag
* `00000 10001` (`0x11`): (optional) start of attributes array
* `00000 11000` (`0x18`): start of data (tag's inner HTML)
* `00000 11001` (`0x19`): end of tag

#### Basic tag structure
Here are some examples of tag encoded structures step-by-step:

Let's take the following HTML code:
```html
<p class="hello"><b>Hello</b>, <u>World</u>!</p>
<em></em>
```

the first thing is to know how to encode letters:
* `p` is encoded as `00001`
* `c` is encoded as `00010`
* `l` is encoded as `00011`
* `a` is encoded as `00100`
* `s` is encoded as `00101`
* `b` is encoded as `00110`
* `u` is encoded as `00111`
* `e` is encoded as `01000`
* `m` is encoded as `01001`

> [!NOTE]
> The future version of SHDP will not have a fyve-based encoding system nor an arbitrary letter encoding system. It will be generated for each requests/responses, and could be from $2$ to $5$ bits depending on the data.

The first thing is to encode the tag name:
```html
<p
```

The encoded tag name will be:
```plaintext
00000000: 00000 10000 00001
```

The next step is to encode the attributes:
```html
class="hello">
```

The encoded attributes will be:
```plaintext
00000000: 00000 10001 00010 00011 00100 00100
00000020: 00000 00000 00000 00000 00101 01101
00000030: 00001 10010 10110 11000 11011 00011
00000040: 01111 00000 11000
```

The next step is to encode the inner HTML:
```html
<b>Hello</b>, <u>World</u>!</p>
```

The encoded inner HTML will be:
```plaintext
00000000: 00000 10000 00110 00000 11000 00000
00000010: 00000 00000 00000 00101 01001 00001
00000020: 10010 10110 11000 11011 00011 01111
00000030: 00000 11001 00000 00000 00000 00000
00000040: 00101 00101 10000 10000 00000 01000
00000050: 00011 10000 01100 00000 00000 00000
00000060: 00000 00010 10101 01110 11011 11011
00000070: 10010 01101 10001 10010 00000 01100
00000080: 10000 00000 00000 00000 00000 10010
00000090: 00010 00001 1001
```

The last step is to encode the second tag:
```html

<em></em>
```

The encoded tag will be:
```plaintext
00000000: 00000 00000 00000 00000 00001 00001
00000010: 01000 00010 00001 00001 00100 00011
00000020: 00000 00011 001
```

So, the final encoded HTML file will be:
```plaintext
00000000: 00000 10000 00001 00000 10001 00010
00000010: 00011 00100 00100 00000 00000 00000
00000020: 00000 00101 01101 00001 10010 10110
00000030: 11000 11011 00011 01111 00000 11000
00000040: 00000 10000 00110 00000 11000 00000
00000050: 00000 00000 00000 00101 01001 00001
00000060: 10010 10110 11000 11011 00011 01111
00000070: 00000 11001 00000 00000 00000 00000
00000080: 00101 00101 10000 10000 00000 01000
00000090: 00011 10000 01100 00000 00000 00000
000000A0: 00000 00010 10101 01110 11011 11011
000000B0: 10010 01101 10001 10010 00000 01100
000000C0: 10000 00000 00000 00000 00000 10010
000000D0: 00010 00001 10010 00000 00000 00000 
000000E0: 00000 00010 00010 10000 00100 00010 
000000F0: 0001 001000 00110 00000 00110 01000
```

Or in hexadecimal:
```plaintext
00000000: 04 02 08 88 64 20 00 00 15 A1 95 B1 B1 BC 18 04  ...êd ...íò▒▒╝..
00000010: 0C 0C 00 00 01 52 19 5B 1B 1B C1 90 00 00 29 61  .....R.[..┴É..)a
00000020: 00 20 70 60 00 00 0A AE DE E4 D8 C8 0C 80 00 00  . p`..«▐Σ╪╚..Ç..
00000030: 48 41 90 00 00 08 50 20 84 83 00 C8              HAÉ..P äâ.╚
```

Here, the data length is 60 bytes. The raw data length is 58 bytes. The compression is relevant on big files.

> [!NOTE]
> In future versions of SHDP, the fyve-based encoding system will be replaced by a more efficient system. This system will use less bits or more bits depending on the data. So, in this context, the data length will be the raw data length or less.

