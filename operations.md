```
 n    op            name                          description                       semantics
────────────────────────────────────────────────────────────────────────────────────────────────────
  0   nop   no operation            Do nothing
  1   nul   put 0                   Put 0 on the stack                        <- 0
  2   put   put                     Put a value on the stack                  <- a
  3   pop   pop                     Pop a value from the stack                -> z
  4   dup   duplicate               Duplicate a value in the stack            -> z; <- z; <- z
  5   cal   function call           Call a function by value                  -> z; call z
  6   ret   return                  Return from the current function          ret
  7   jmp   jump                    Set the instruction pointer by value      I = a
  8   rd1   read 1 byte             Read a byte by pointer                    -> z; <- 1[z]
  9   rd2   read 2 bytes            Read two bytes by pointer                 -> z; <- 2[z]
 10   rd4   read 4 bytes            Read four bytes by pointer                -> z; <- 4[z]
 11   wr1   write 1 byte            Write a byte by pointer                   -> z; -> y; 1[z] = y
 12   wr2   write 2 bytes           Write two bytes by pointer                -> z; -> y; 2[z] = y
 13   wr4   write 4 bytes           Write four bytes by pointer               -> z; -> y; 4[z] = y
 14   loc   local pointer           Take a local pointer                      <- F + a
 15   ld1   load 1 byte             Load a byte by local pointer              <- 1[F + a]
 16   ld2   load 2 bytes            Load two bytes by local pointer           <- 2[F + a]
 17   ld4   load 4 bytes            Load four bytes by local pointer          <- 4[F + a]
 18   st1   store 1 byte            Store a byte by local pointer             -> z; 1[F + a] = z
 19   st2   store 2 bytes           Store two bytes by local pointer          -> z; 2[F + a] = z
 20   st4   store 4 bytes           Store four bytes by local pointer         -> z; 4[F + a] = z
 21   alc   allocate                Allocate new memory                       -> z; <- alloc z
 22   rlc   reallocate              Reallocate a memory                       -> z; <- realloc z
 23   dlc   deallocate              Deallocate a memory                       -> z; dealloc z
 24   flc   allocate frame          Allocate new memory in the current        -> z; falloc z
                                    stackframe
 25   mov   move                    Move a memory from a pointer to other     -> z; -> y; -> x;
                                    by length                                 move z y x
 26   inc   increment               Increment a value                         -> z; <- z + 1
 27   dec   decrement               Decrement a value                         -> z; <- z - 1
 28   neg   negation                Negate a value                            -> z; <- -z
 29   add   addition                Add values                                -> z; -> y; <- z + y
 30   sub   subtraction             Subtract values                           -> z; -> y; <- z - y
 31   mul   multiplication          Multiply values                           -> z; -> y; <- z * y
 32   sml   signed multiplication   Multiply signed values                    -> z; -> y; <- z ~* y
 33   div   division                Divide values                             -> z; -> y; <- z / y
 34   sdv   signed division         Divide signed values                      -> z; -> y; <- z ~/ y
 35   rem   remainder               Divide values with remainder              -> z; -> y; <- z % y
 36   srm   signed remainder        Divide signed values with remainder       -> z; -> y; <- z ~% y
 37   flt   float operation         Perform a float operation                 # todo
 38   lsh   left shift              Bitwise shift value to left               -> z; -> y; <- z << y
 39   rsh   right shift             Bitwise shift value to right              -> z; -> y; <- z >> y
 40   ban   bitwise and             Bitwise and on values                     -> z; -> y; <- z & y
 41   bor   bitwise or              Bitwise or on values                      -> z; -> y; <- z | y
 42   bxr   bitwise xor             Bitwise xor on values                     -> z; -> y; <- z ^ y
 43   bnt   bitwise not             Bitwise not on a value                    -> z; <- !z
 44   iev   if equal to value       Execute next if equal to value            -> z; if z == a
 45   inv   if not equal to value   Execute next if not equal to value        -> z; if z != a
 46   ieq   if equal                Execute next if values are equal          -> z; -> y; if z == y
 47   ine   if not equal            Execute next if values are not equal      -> z; -> y; if z != y
 48   ilt   if less                 Execute next if less                      -> z; -> y; if z < y
 49   igt   if greater              Execute next if greater                   -> z; -> y; if z > y
 50   ile   if less or equal        Execute next if less or equal             -> z; -> y; if z <= y
 51   ige   if greater or equal     Execute next if greater or equal          -> z; -> y; if z >= y
 52   val   value                   Define a value block                      def values
 53   fun   function                Define a function                         def function a
 54   imp   import                  Define an import block                    def imports
 55   exp   export                  Define an export block                    def exports 
```
