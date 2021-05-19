# TEC 8 编译器

由于北京邮电大学使用了<del>基于大数据与多方协同人工智能自主研发国产可控分布式集成</del>TEC-8 实验平台，我又是一个记性很差，容易导致颅内资料消失的蒟蒻，于是写了这么个东西来帮我杀死 TEC-8 的汇编。一方面是预习实验，另一方面还可以复习下自动机模型。

为什么要用 Rust 呢？别问，问就是喜欢

## 指令集

<del>你邮要是经费还是 21 年这个样，这玩意大概到 114514 届也不会变</del>

[![g5nkFO.md.png](https://z3.ax1x.com/2021/05/19/g5nkFO.md.png)](https://imgtu.com/i/g5nkFO)

## 使用

类似于 gcc，

```bash
./tec-8-compiler -o [FILE] <INPUT>
```

## 输出

由于需要手动操作，作者并没有直接输出二进制文件，而是采用二进制码+指令的格式：

```x86asm
0001:  01010011 LD R0, [R3]
0010:  01001100 INC R3
0011:  01010111 LD R1,[R3]
0100:  00100001 SUB R0,R1
0101:  10001010 JZ 0BH
0110:  01100010 ST R0,[R2]
0111:  01001100 INC R3
1000:  01010011 LD R0,[R3]
1001:  00010001 ADD R0,R1
1010:  01111110 JC 0CH
1011:  01001000 INC R2
1100:  01101010 ST R2,[R2]
1101:  00110001 AND R0,R1
1110:  10100010 OUT R2
1111:  11100000 STP
```