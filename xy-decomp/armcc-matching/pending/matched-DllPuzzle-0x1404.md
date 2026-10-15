Module: DllPuzzle
Offset: 0x1404
Size: 0x30
Match: full masked
One-line: dtor-stage variant: store vt arg, call base cleanup, fn260(fn1c8(o->p4->f14),17), tail-call fn268(o).
Note: tail-calls base dtor stage (fn268) returning void*; vtable ptr arrives as 2nd arg (pass-through), unlike sibling 0x9310.
