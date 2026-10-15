Module: DllSequence
Offset: 0x1140
Size: 0x18
Match: 6/6
One-line: Sets a single byte flag deep inside a buffer (`this->buf[0x181d] = 1`), computed as two sequential pointer-cursor adds.
Note: Offset 0x181d (6173) exceeds strb's 12-bit immediate range, forcing a split either way, but armcc's natural single-expression split (fold as much as fits into the strb offset: `add #0x1000; strb [r0,#0x81d]`) doesn't match retail's split (`add #0x1800; add #29; strb [r0,#0]`). Only a literal sequential-cursor idiom (`p = buf; p += 0x1800; p += 29; *p = 1;`) reproduced it — same cursor-store pattern already documented for DllNumberInput@0x18fc, now seen in a plain field-set (not a constructor).
