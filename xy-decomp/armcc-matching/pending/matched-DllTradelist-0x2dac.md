Module: DllTradelist
Offset: 0x2dac
Size: 0xc
Match: 3/3
One-line: Prologue-less tail-call forwarder — sets up two literal args (r1=22, r2=1), passes r0 through unchanged, tail-branches to a local function.
Note: New forwarder-with-fixed-args idiom variant (prior forwarders seen were argument-less). A plain `return local_fn(a, 22, 1);` with no prologue matched on the first try since the whole body is leaf/tail-call shaped.
