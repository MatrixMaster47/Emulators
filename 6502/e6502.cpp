#include "e6502.hpp"
#include <cstdint>

using namespace EMU6502;

bool CpuData::AdvanceClock() {
	// Instruction is CpuData::Mem[PC]
	switch(Mem[PC]) {
		normal:
			return false;
			break;
		case (uint8_t)Opcode::I_ADC:
			A += Mem[PC+1] + PS[FlagC];
			return true;
			break;
	}
}
