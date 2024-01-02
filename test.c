#include <stdio.h>
#include "bindings.h"

int onDataCallback(uint8_t *data, size_t len) {
    printf("%s", data);
    return 0;
}




int main() {
    unsigned long result = add(4, 5);  // 调用 Rust 函数
    printf("The result is: %lu\n", result);
    char *url ="aaaa";
    urldecode(url, 4);
    return 0;
}
