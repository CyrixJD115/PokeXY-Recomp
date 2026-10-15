Module: DllGts
Offset: 0x9880
Size: 0x18
Attempted: this->f0c (byte*) = 10; this->f3c = 1. armcc emits mov r1,#1 before the strb of 10 (order mismatch at word 0x988c: retail hoists mov r2,#10 / mov r1,#1 both before the first store, armcc interleaves). 5/6 words. Try reordering stores or volatile-free temp variants.
