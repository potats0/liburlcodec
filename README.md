# 简介
该项目属于实验项目，以验证rust与waf的可融合性。

# 编译过程
1. rust编译
   `cargo build --release`
2. 生成头文件
    `cbindgen --config cbindgen.toml --crate liburlcodec --output liburlcodec.h --lang c`

# 如何在c语言中调用？
```
#include <stdio.h>
#include "liburlcodec.h"
int main() {
    unsigned long result = add(4, 5);  // 调用 Rust 函数
    printf("The result is: %lu\n", result);
    return 0;
}
```
gcc 编译命令 

`gcc -o test test.c -L target/release/ -lurlcodec`

* -L 是rust编译产物（shared library）所在的目录
* -l 是rust编译产物的名称，去掉文件名前lib这三个字母（gcc会自动帮你加上）

# 如何调试？
1. 请针对你的函数，编写详细的unit test文档。
2. 在vs code中，点击debug tests即可。这样不需要借助GDB一类的调试器，在vscode中借助于rust，查看你断点处的各种变量
3. 如果还是不可以，请参考https://juejin.cn/s/vscode%20debug%20rust%20unit%20test

# 参考文档
1. https://github.com/mozilla/cbindgen
