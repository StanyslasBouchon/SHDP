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
<pre><span style="color: rgb(102, 102, 102);">00000000</span> <span style="color: rgb(36, 114, 200)">01</span> <span class="magenta">00 01</span> <span class="yellow">00 00 00 68</span> <span class="bright-black">48 65 6C 6C 6F 2C 20 57 6F ......hHello, Wo
00000010 72 6C 64 21                                     rld!</span>
</pre>

In blue (`rgb(36, 114, 200)`), the version.<br>
In magenta (`rgb(188, 63, 188)`), the event code.<br>
In yellow (`rgb(229, 229, 16)`), the data length.


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
In bright red (`rgb(241, 76, 76)`), the operating codes.<br>
In blue (`rgb(36, 114, 200)`), the encoded letters.<br>
In bright green (`rgb(35, 209, 139)`), the UTF-8 chains.
In bright yellow (`rgb(245, 245, 67)`), the UTF-8 chain length.

The encoded tag name will be:
<pre><span class="bright-red">00000 10000</span> <span class="blue">00001</span></pre>

The next step is to encode the attributes:
```html
class="hello">
```

The encoded attributes will be:
<pre><span class="bright-red">00000 10001</span> <span class="blue">00010 00011 00100 00100</span>
<span class="bright-red">00000 00000</span> <span class="bright-yellow">00000 00000 00101</span> <span class="bright-green">01101
00001 10010 10110 11000 11011 00011
01111</span> <span class="bright-red">00000 11000</span></pre>

The next step is to encode the inner HTML:
```html
<b>Hello</b>, <u>World</u>!</p>
```

The encoded inner HTML will be:
<pre><span class="bright-red">00000 10000</span> <span class="blue">00110</span> <span class="bright-red">00000 11000 00000
00000</span> <span class="bright-yellow">00000 00000 00101</span> <span class="bright-green">01001 00001
10010 10110 11000 11011 00011 01111</span>
<span class="bright-red">00000 11001 00000 00000</span> <span class="bright-yellow">00000 00000
00101</span> <span class="bright-green">00101 10000 10000 0</span><span class="bright-red">0000 01000
0</span><span class="blue">0011 1</span><span class="bright-red">0000 01100 00000 00000 0</span><span class="bright-yellow">0000
00000 00010 1</span><span class="bright-green">0101 01110 11011 11011
10010 01101 10001 10010 0</span><span class="bright-red">0000 01100
10000 00000 0</span><span class="bright-yellow">0000 00000 00000 1</span><span class="bright-green">0010
0001</span><span class="bright-red">0 00001 1001</span>
</pre>

The last step is to encode the second tag:
```html

<em></em>
```

The encoded tag will be:
<pre><span class="bright-red">00000 00000</span> <span class="bright-yellow">00000 00000 00001</span> <span class="bright-green">00001
010</span><span class="bright-red">00 00010 000</span><span class="blue">01 00001 001</span><span class="bright-red">00 00011
00000 00011 001</span>
</pre>

So, the final encoded HTML file will be:
<pre><span class="bright-red">00000 10000</span> <span class="blue">00001</span> <span class="bright-red">00000 10001</span> <span class="blue">00010
00011 00100 00100</span> <span class="bright-red">00000 00000</span> <span class="bright-yellow">00000
00000 00101</span> <span class="bright-green">01101 00001 10010 10110
11000 11011 00011 01111</span> <span class="bright-red">00000 11000</span>
<span class="bright-red">00000 10000</span> <span class="blue">00110</span> <span class="bright-red">00000 11000 00000
00000</span> <span class="bright-yellow">00000 00000 00101</span> <span class="bright-green">01001 00001
10010 10110 11000 11011 00011 01111</span>
<span class="bright-red">00000 11001 00000 00000</span> <span class="bright-yellow">00000 00000
00101</span> <span class="bright-green">00101 10000 10000 0</span><span class="bright-red">0000 01000
0</span><span class="blue">0011 1</span><span class="bright-red">0000 01100 00000 00000 0</span><span class="bright-yellow">0000
00000 00010 1</span><span class="bright-green">0101 01110 11011 11011
10010 01101 10001 10010 0</span><span class="bright-red">0000 01100
10000 00000 0</span><span class="bright-yellow">0000 00000 00000 1</span><span class="bright-green">0010
0001</span><span class="bright-red">0 00001 1001</span><span class="bright-red">0 00000 0000</span><span class="bright-yellow">0 00000 
00000 0001</span><span class="bright-green">0 00010 10</span><span class="bright-red">000 00100 00</span><span class="blue">010 
0001 001</span><span class="bright-red">000 00110 00000 00110 01</span><span class="bright-black">000</span></pre>

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

