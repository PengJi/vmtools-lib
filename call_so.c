// compile: gcc -g main.c -o call_rust -lfoo -L./linux/target/debug/
// run: LD_LIBRARY_PATH=/root/codestore/lang/rust/api_lib/linux/target/debug ./call_rust

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern int check_svt_process();

int main() {
    // svt
    int check_res = check_svt_process();
    printf("check svt process result: %d\n", check_res);

    return 0;
}
