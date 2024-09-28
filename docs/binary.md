objdump - display information from object files.

object file formats:
- ELF
- COFF (PE格式的前期, .obj, .lib)
- PE: Portable Executable (Windows：可执行文件、DLL)
- Mach-O
- a.out (Assembler Ouput)

objdump 更适合代码级别的分析和反汇编
readelf 更适合文件结构和原数据的分析
nm: 列出目标文件的符号
strings: 提出二进制文件中的可打印字符串
ldd: 显示程序或者共享库所依赖的共享库
file: 确定文件类型
ltrace: 跟踪程序的库函数调用
strace: 跟踪函数的系统调用