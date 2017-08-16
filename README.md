# gather-cidr-map

`gather-cidr-map` collects IP (prefix) -> value mappings from stdin,
gathers for each IP the last mapped value and outputs the mappings
either grouped into the smallest number of prefixes (`-p`, the default)
or `from-to` ranges (`-r`).

It can also output IPs not mapped to any value (only in the prefix
output mode, not for ranges) when using the `-u` option.

The output format is either:
* `a-b\tvalue` for a range mapping
* `a/n\tvalue` for a prefix mapping
* `any\tvalue` for a range or prefix mapping for all IPv4+IPv6 IPs
* the `\tvalue` suffix is missing if there was no value set (note that
  the value is prefixed by a tab in the output)

The input is similar:
* input is split into lines
* on each line beginning and trailing whitespace is removed
* empty lines and lines starting with a `#` are ignored
* the line is split at the first whitespace block into IP (prefix) and
  value; if there is no whitespace the value is an empty string.

To simulate the behaviour of a routing table ("best match wins") you
should sort the input to start with large prefixes (i.e. small numbers
after the `/`).

## Build

Clone with git, then run `cargo build` or `cargo build --release`.  The
binary will be in `./target/debug/` or `./target/release/`.

## Example

Input:

```
0.0.0.0/0        normal
10.0.0.0/8       private
172.16.0.0/12    private
192.168.0.0/16   private
```

Output: (without any options)

```
0.0.0.0/5	normal
8.0.0.0/7	normal
10.0.0.0/8	private
11.0.0.0/8	normal
12.0.0.0/6	normal
16.0.0.0/4	normal
32.0.0.0/3	normal
64.0.0.0/2	normal
128.0.0.0/3	normal
160.0.0.0/5	normal
168.0.0.0/6	normal
172.0.0.0/12	normal
172.16.0.0/12	private
172.32.0.0/11	normal
172.64.0.0/10	normal
172.128.0.0/9	normal
173.0.0.0/8	normal
174.0.0.0/7	normal
176.0.0.0/4	normal
192.0.0.0/9	normal
192.128.0.0/11	normal
192.160.0.0/13	normal
192.168.0.0/16	private
192.169.0.0/16	normal
192.170.0.0/15	normal
192.172.0.0/14	normal
192.176.0.0/12	normal
192.192.0.0/10	normal
193.0.0.0/8	normal
194.0.0.0/7	normal
196.0.0.0/6	normal
200.0.0.0/5	normal
208.0.0.0/4	normal
224.0.0.0/3	normal
```
