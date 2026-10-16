/*
 * DllBattle @ 0x89aac, size 0x20 -- script-command handler: reads one argument from the script context into a discarded local (rd(ctx,&v,1)), then forwards the battle object to a worker.
 * Masked match 8/8. Script-arg reader shape (veneer 0x504 = int arg fetch); sibling family of 0x86734/0x84af0/0x8638c/0x86bcc.
 * Style: natural
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllBattle 0x89aac 0x20 xy-decomp/armcc-matching/matched/DllBattle/0x89aac.c
 */
extern void rd(int, int*, int);
extern void fnB(int);
void fn(int a, int b) { int v; rd(a, &v, 1); fnB(b); }
