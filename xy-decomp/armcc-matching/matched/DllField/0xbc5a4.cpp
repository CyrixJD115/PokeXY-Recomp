// DllField@0xbc5a4, size 0x30, 12/12 masked match.
// Secondary-base deleting-dtor stage (one of 25 identical sites): this->vptr = vt (arg r1, volatile store); if (this->m@+32) Free(this, &this->m); then tail-calls operator delete on Base(this+8) - 8.
// Style: trick(volatile-store) -- vptr store via volatile pointer pins the mov r4,r0 copy ahead of the store; Reset() as inline C++ member (see 0xb1bb8.cpp)
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0xbc5a4 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0xbc5a4.cpp

struct S;
extern void fr(S *self, void *p);
extern void *g(void *p);
extern void del(void *p);
struct S {
    void *vt;
    char pad[28];
    void *m;
    void Reset() { if (m) fr(this, &m); }
    S *Sub() { return (S *)((char *)this + 8); }
};
void f(S *o, void *vt)
{
    *(void *volatile *)o = vt;
    o->Reset();
    del((char *)g(o->Sub()) - 8);
}
