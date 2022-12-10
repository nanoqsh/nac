```
 n    op            name                          description                       semantics
────────────────────────────────────────────────────────────────────────────────────────────────────
  0   nop   no operation            Do nothing
  1   nul   put nul                 Put zero value on the stack               <- 0
  2   put   put                     Put a value on the stack                  <- a
  3   pop   pop                     Pop a value from the stack                -> z
  4   dup   duplicate               Duplicate a value in the stack            -> z; <- z; <- z
  5   cal   function call           Call a function by value                  call a
  6   inv   function invoke         Call a function from the stack            -> z; invoke z
  7   ret   return                  Return from the current function          ret
  8   jmp   jump                    Set the instruction pointer by value      I = a
  9   rd1   read 1 byte             Read a byte by pointer                    -> z; <- 1[z]
 10   rd2   read 2 bytes            Read two bytes by pointer                 -> z; <- 2[z]
 11   rd4   read 4 bytes            Read four bytes by pointer                -> z; <- 4[z]
 12   wr1   write 1 byte            Write a byte by pointer                   -> z; -> y; 1[z] = y
 13   wr2   write 2 bytes           Write two bytes by pointer                -> z; -> y; 2[z] = y
 14   wr4   write 4 bytes           Write four bytes by pointer               -> z; -> y; 4[z] = y
 15   loc   local pointer           Take a local pointer                      <- F + a
 16   ld1   load 1 byte             Load a byte by local pointer              <- 1[F + a]
 17   ld2   load 2 bytes            Load two bytes by local pointer           <- 2[F + a]
 18   ld4   load 4 bytes            Load four bytes by local pointer          <- 4[F + a]
 19   st1   store 1 byte            Store a byte by local pointer             -> z; 1[F + a] = z
 20   st2   store 2 bytes           Store two bytes by local pointer          -> z; 2[F + a] = z
 21   st4   store 4 bytes           Store four bytes by local pointer         -> z; 4[F + a] = z
 22   alc   allocate                Allocate new memory                       -> z; <- alloc z
 23   rlc   reallocate              Reallocate a memory                       -> z; <- realloc z
 24   dlc   deallocate              Deallocate a memory                       -> z; dealloc z
 25   flc   allocate frame          Allocate new memory in the current        -> z; falloc z
                                    stackframe
 26   mov   move                    Move a memory from a pointer to other     -> z; -> y; -> x;
                                    by length                                 move z y x
 27   inc   increment               Increment value in the stack              -> z; <- z + 1
 28   dec   decrement               Decrement value in the stack              -> z; <- z - 1
 29   neg   negation                Negate value in the stack                 -> z; <- -z
 30   add   addition                Add values                                -> z; -> y; <- z + y
 31   sub   subtraction             Subtract values                           -> z; -> y; <- z - y
 32   mul   multiplication          Multiply values                           -> z; -> y; <- z * y
 33   sml   signed multiplication   Multiply signed values                    -> z; -> y; <- z ~* y
 34   div   division                Divide values                             -> z; -> y; <- z / y
 35   sdv   signed division         Divide signed values                      -> z; -> y; <- z ~/ y
 36   rem   remainder               Divide values with remainder              -> z; -> y; <- z % y
 37   srm   signed remainder        Divide signed values with remainder       -> z; -> y; <- z ~% y
 38   flt   float operation         Perform a float operation                 # todo
 39   lsh   left shift              Bitwise shift value to left               -> z; -> y; <- z << y
 40   rsh   right shift             Bitwise shift value to right              -> z; -> y; <- z >> y
 41   ban   bitwise and             Bitwise and on values                     -> z; -> y; <- z & y
 42   bor   bitwise or              Bitwise or on values                      -> z; -> y; <- z | y
 43   bxr   bitwise xor             Bitwise xor on values                     -> z; -> y; <- z ^ y
 44   bnt   bitwise not             Bitwise not on a value                    -> z; <- !z
 45   iez   if equal to 0           Execute next if value is zero             -> z; if z == 0
 46   inz   if not equal to 0       Execute next if value is not zero         -> z; if z != 0
 47   ieq   if equal                Execute next if values are equal          -> z; -> y; if z == y
 48   ine   if not equal            Execute next if values are not equal      -> z; -> y; if z != y
 49   ilt   if less                 Execute next if less                      -> z; -> y; if z < y
 50   igt   if greater              Execute next if greater                   -> z; -> y; if z > y
 51   ile   if less or equal        Execute next if less or equal             -> z; -> y; if z <= y
 52   ige   if greater or equal     Execute next if greater or equal          -> z; -> y; if z >= y
 53   val   value                   Define a value block                      def values
 54   fun   function                Define a function                         def function a
 55   imp   import                  Define an import block                    def imports
 56   exp   export                  Define an export block                    def exports
```
