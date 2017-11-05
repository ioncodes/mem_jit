# mem_tests

Playing around with memory regions. This creates a read/write region which can be used as variable store for a JIT running in a read/write/execute memory block. It can be used for arrays and stuff. This allows us to use JIT without fiddling around with the registers.