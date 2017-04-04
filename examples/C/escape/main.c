#include <stdio.h>
#include "libdbgp.h"

extern uint16_t dbgp_escape_string(uint8_t);

int main() {
	printf("%i\n", dbgp_escape_string(8));
	return 0;
}
