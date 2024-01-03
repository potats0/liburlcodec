#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "bindings.h"


int main() {
    char *url ="%25%36%31%25%36%32%25%36%33%25%36%34%25%36%35%25%36%34";
    char *src = malloc(100);
    memcpy(src, url, strlen(url));
    printf("raw: %s\n", src);
    urldecode(src, strlen(url));
    printf("result: %s\n", src);
    return 0;
}
