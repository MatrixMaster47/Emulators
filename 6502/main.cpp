#include "e6502.hpp"
#include <iostream>

int main() {
	EMU6502::CpuData cpu;
	uint8_t val = 235;
	std::cout << "should be 235: ";
	std::cout << (int)cpu.AddrImm(val) << std::endl;
}
