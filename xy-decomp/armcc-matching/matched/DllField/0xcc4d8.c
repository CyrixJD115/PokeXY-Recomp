// DllField@0xcc4d8, size 0x6c, 27/27 masked match.
// State-handler: A(this->[+68],1); if Cur(sub@+184) != 103 returns 37; else Go(this), fetches a 16-byte struct via Get(sub) and passes its address to Use(obj, &v) (0xb0334); returns 1.
// Style: natural -- 16-byte struct returned into a local, passed by address
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xcc4d8 0x6c \
//       xy-decomp/armcc-matching/matched/DllField/0xcc4d8.c

struct V { int a, b, c, d; };
extern void A(void *, int);
extern int Cur(void *);
extern void Go(void *);
extern struct V Get(void *);
extern void Use(void *, struct V *);
int Fn(char *t) {
    struct V w;
    A(*(void **)(t + 68), 1);
    if (Cur(*(char **)(t + 68) + 184) != 103) return 37;
    Go(t);
    w = Get(*(char **)(t + 68) + 184);
    Use(*(void **)(t + 68), &w);
    return 1;
}
