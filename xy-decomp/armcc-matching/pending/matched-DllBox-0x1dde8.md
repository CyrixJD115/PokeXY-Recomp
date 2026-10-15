Module: DllBox
Offset: 0x1dde8
Size: 0x48
Match: 18/18
One-line: init helper storing arg field into sub-object, two veneer calls, returns 1
Note: arg->f must be loaded once into a void* local and reused (store + call arg).
