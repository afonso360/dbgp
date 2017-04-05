#include <stdio.h>
#include "libdbgp.h"

extern const char* dbgp_escape_string(const char*);

int main() {
	printf("%s\n", dbgp_escape_string("e\"scaped"));
	return 0;
}
