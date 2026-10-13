# Gemini task 5b: fix the map blob layout (follow-up to task 5)

Output folder: `F:\Projects\Decomp\hackgu-rs\gemini_out5b\`. Scripts in `tools\gem5\`. Read-only on everything
else. Do NOT edit `crates\`, `HANDOFF.md` or `assets\`, and do NOT create git branches or commits.

## What Claude found wrong in gemini_out5 (see HANDOFF.md "Review of Gemini task 5")
1. Grid 1 at payload+0x14 cannot be right. With stride 40 the land wraps across row ends: rows 18-20 run off column 39
   and continue at column 0. Rows are only contiguous when they start at payload offset 32..39 (mod 40).
   Most likely `rax` (the return of 0x1801620A0 = `[obj+0x38]`) points 16 bytes into the payload.
2. "Grid 2 = 200x200 at +0x654" overruns the 41,604-byte blob.
3. Value 1 (arrival Platform) lands on a sea cell of your own grid 1.

## Do this
A. Trace where `[obj+0x38]` is set for a BinaryBlob (0x2400) object: find the CCS section loader for type 0x2400
   and quote the instruction that stores the data pointer, with the offset it adds. State exactly which file byte
   `rax` points to for m101.ccs.
B. With that base, give the real grid 1 start, its dimensions and stride. Check it: print the m101 grid; it must not
   wrap, and every object must sit on or next to land.
C. Find grid 2's real dimensions from the code. Quote the loop bounds and the multiply/stride instructions in the
   loop at ~0x1806741E0. Recompute the six m101 objects (bytes 18116, 22056, 22091, 25133, 25134, 26106 of our payload,
   which starts at the blob's first byte `00 01 00 00`) into grid-2 and grid-1 coordinates.
D. For each handler in the jump table at 0x1806741F3, say only what the code shows: the function called, the fields
   written, and the strings or model names it references. Use no invented labels. If you cannot tell what a value
   does, write UNKNOWN.
E. Redo m101_decoded.txt and maps.csv with the corrected layout. Mark every claim VERIFIED (with the raw bytes or
   instruction) or GUESS.

Skip the procedural generator (Part C of task 5) until A-C are right.
