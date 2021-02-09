#include "e6502.hpp"
#include <cstdint>

using namespace EMU6502;

bool CpuData::AdvanceClock() {
	// Instruction is CpuData::Mem[PC]
	switch(Mem[PC]) {
		normal:
			return Result::Failure;
			break;
		case Opcode::I_ADC:
			A += Mem[PC+1] + PS[FlagC];
			return true;
			break;
	}
}

inline uint8_t* CpuData::AddrAcc() {
	return &A;
}
inline uint8_t EMU6502::CpuData::AddrImm(uint8_t& Value) {
	return Value;
}
inline uint8_t* CpuData::AddrZpg(uint8_t& Value) {
	return &Mem[Value];
}
inline uint8_t* CpuData::AddrZpX(uint8_t& Value) {
	return &Mem[(uint8_t)Value + X];
}
inline uint8_t* CpuData::AddrZpY(uint8_t& Value) {
	return &Mem[(uint8_t)Value + Y];
}
inline uint8_t* CpuData::AddrRel(uint8_t& Value ) {
	return &Mem[PC + *(int8_t*)&Value];
}
inline uint8_t* CpuData::AddrAbs(uint8_t& Low, uint8_t& High) {
	return &Mem[(uint16_t)(High << 8) + Low];
}
inline uint8_t* CpuData::AddrAbX(uint8_t& Low, uint8_t& High) {
	return &Mem[(uint16_t)(High << 8) + Low + X];
}
inline uint8_t* CpuData::AddrAbY(uint8_t& Low, uint8_t& High) {
	return &Mem[(uint16_t)(High << 8) + Low + Y];
}
inline uint8_t* CpuData::AddrInd(uint8_t& Low, uint8_t& High) {
	return &Mem[*AddrAbs(Low, High)];
}
inline uint8_t* CpuData::AddrInX(uint8_t& Low, uint8_t& High) {
	return &Mem[*AddrAbs(Low, High) + X];
}
inline uint8_t* CpuData::AddrInY(uint8_t& Low, uint8_t& High) {
	return &Mem[*AddrAbs(Low, High) + Y];
}
