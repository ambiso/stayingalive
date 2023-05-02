#include <stdio.h>
#include "TestU01.h"
#include "unif01.h"
#include "broken_prng.h"

static MyRng *rng = 0;

uint32_t my_rng_wrapper() {
    if (rng == 0) {
        rng = rng_new();
    }
    return rng_gen(rng);
}

int main() {
    unif01_Gen *gen = unif01_CreateExternGenBits("My RNG", my_rng_wrapper);
    gen->param = rng;
    bbattery_BigCrush(gen);
    unif01_DeleteExternGenBits(gen);
    rng_free(rng);
    return 0;
}
