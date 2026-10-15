extern int X1, X2, X3, X4, X5;
extern int *g;
int entry(int *p, unsigned op) {
    switch (op) {
    case 0: *p = (int)&X1; break;
    case 1: g = p; break;
    case 2: *p = (int)g; break;
    case 3: p[0] = (int)&X2; p[1] = (int)&X3; break;
    case 4: p[0] = (int)&X4; p[1] = (int)&X5; break;
    default: return 0xd8e12c21;
    }
    return 0;
}
