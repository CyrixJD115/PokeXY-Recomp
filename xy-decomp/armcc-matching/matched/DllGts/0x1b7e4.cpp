// DllGts @0x1b7e4 (size 0x8) — masked match 2/2.
//
// Named export `_ZN9xy_system2bg6System13SetIsCallDrawEb` demangles to
// `xy_system::bg::System::SetIsCallDraw(bool)` — a trivial boolean-field
// setter (classic field-setter idiom: `strb r1,[r0,#N]; bx lr`).
//
// The retail bytes put the flag at byte offset 0xd9 (217) of the object;
// modeled here as a padding array of the right size followed by the real
// member, which reproduces the exact `strb r1,[r0,#217]` the retail
// binary has.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
//       DllGts 0x1b7e4 0x8 \
//       xy-decomp/armcc-matching/matched/DllGts/0x1b7e4.cpp

namespace xy_system {
namespace bg {

class System {
    unsigned char pad[217]; // offsets 0x00..0xd8
    bool is_call_draw;      // offset 0xd9

public:
    void SetIsCallDraw(bool v);
};

void System::SetIsCallDraw(bool v) {
    is_call_draw = v;
}

} // namespace bg
} // namespace xy_system
