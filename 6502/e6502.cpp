#include "e6502.hpp"
#include <cstdint>

using namespace EMU6502;

bool CpuData::AdvanceClock() {
	// Instruction is CpuData::Mem[PC]
	switch(Ins++) {
		normal:
			return false;
			break;
		case (0x69<<4)|0: break;
		case (0x69<<4)|1: (A += AddrImm(Mem[PC])) < 255 ? PS[FlagC] = 0 : PS[FlagC] = 1; break;
	}
	PC++;
	return true;
}
