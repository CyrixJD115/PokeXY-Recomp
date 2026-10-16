// DllField@0x5b29c, size 0x1c, 7/7 masked match.
// Getter: G(a)->+12->+28, then ptr@+384, returns signed byte ptr[1].
// Plain pointer chase.
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5b29c 0x1c \
//       xy-decomp/armcc-matching/matched/DllField/0x5b29c.c

extern char *G(void *);
signed char Fn(void *a) {
    char *p = *(char **)(*(char **)(G(a) + 12) + 28);
    return (*(signed char **)(p + 384))[1];
}
