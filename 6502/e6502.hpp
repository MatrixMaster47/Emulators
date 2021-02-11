#pragma once
#include <cstdint>
#include <bitset>
#include <vector>

namespace EMU6502 {
	// For accessing the flags in the PS register
	const uint8_t FlagC = 0;
	const uint8_t FlagZ = 1;
	const uint8_t FlagI = 2;
	const uint8_t FlagD = 3;
	const uint8_t FlagB = 4;
	const uint8_t FlagV = 6;
	const uint8_t FlagN = 7;

	enum class Opcode : uint8_t {
		// Immediate
		I_ADC = 0x69,
		I_AND = 0x29, 
		I_CMP = 0xC9,
		I_CPX = 0xE0,
		I_CPY = 0xC0,
		I_EOR = 0x49,
		I_LDA = 0xA9,
		I_LDX = 0xA2,
		I_LDY = 0xA0,
		I_ORA = 0x09,
		I_SBC = 0xE9

	};

	class CpuData {
	public:
		// Registers
		uint16_t PC;
		uint8_t  SP, A, X, Y;
		std::bitset<8> PS;

		// Memory (vector seems easier than dealing with a heap array)
		std::vector<uint8_t> Mem;
	
		// Methods
		CpuData() : PC(0), SP(0), A(0), X(0), Y(0), Mem(1024*64, 0) {
			PS.reset();
		}

		bool AdvanceClock();

		inline uint8_t* AddrAcc() {
			return &A;	
		}
		inline uint8_t AddrImm(uint8_t& Value) {
			return Value;	
		}
		inline uint8_t* AddrZpg(uint8_t& Value) {
			return &Mem[Value];
		}
		inline uint8_t* AddrZpX(uint8_t& Value) {
			return &Mem[(uint8_t)Value + X];	
		}
		inline uint8_t* AddrZpY(uint8_t& Value) {
			return &Mem[(uint8_t)Value + Y];	
		}
		inline uint8_t* AddrRel(uint8_t& Value ) {
			return &Mem[PC + *(int8_t*)&Value];	
		}
		inline uint8_t* AddrAbs(uint8_t& Low, uint8_t& High) {
			return &Mem[(uint16_t)(High << 8) + Low];	
		}
		inline uint8_t* AddrAbX(uint8_t& Low, uint8_t& High) {
			return &Mem[(uint16_t)(High << 8) + Low + X];	
		}
		inline uint8_t* AddrAbY(uint8_t& Low, uint8_t& High) {
			return &Mem[(uint16_t)(High << 8) + Low + Y];	
		}
		inline uint8_t* AddrInd(uint8_t& Low, uint8_t& High) {
			return &Mem[*AddrAbs(Low, High)];		
		}
		inline uint8_t* AddrInX(uint8_t& Low, uint8_t& High) {
				return &Mem[*AddrAbs(Low, High) + X];		
		}
		inline uint8_t* AddrInY(uint8_t& Low, uint8_t& High) {
				return &Mem[*AddrAbs(Low, High) + Y];
		}
	};
}
