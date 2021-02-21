#include "e6502.hpp"
#include <iostream>

int main() {
	EMU6502::CpuData cpu;
	cpu.Ins = (0x69<<4)-1;
	cpu.AdvanceClock();
	std::cout << cpu.A << std::endl;
}
