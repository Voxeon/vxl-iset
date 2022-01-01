

use crate::instruction_arguments::{Register, Address, Immediate};
use crate::instruction::Instruction;
use crate::syscall_handler::SyscallHandler;

/*
NOTE: THIS FILE IS AUTOGENERATED.
*/


pub trait ExecuteInstruction {
    type Machine: ExecuteInstruction;
    type Output;

    fn execute_instruction<S: SyscallHandler<Self::Machine>>(&mut self, instruction: Instruction, handler: &mut S) -> Self::Output {
        return match instruction {
        	Instruction::Nop => self.execute_nop(),
        	Instruction::Syscall(i) => self.execute_syscall(handler, i),
        	Instruction::Ldb(i, r) => self.execute_ldb(i, r),
        	Instruction::Ldi(i, r) => self.execute_ldi(i, r),
        	Instruction::Ldf(i, r) => self.execute_ldf(i, r),
        	Instruction::Mov(r, r1) => self.execute_mov(r, r1),
        	Instruction::Push(r) => self.execute_push(r),
        	Instruction::Pop(r) => self.execute_pop(r),
        	Instruction::Sget(r, r1) => self.execute_sget(r, r1),
        	Instruction::Malloc(r, r1) => self.execute_malloc(r, r1),
        	Instruction::Malloci(i, r) => self.execute_malloci(i, r),
        	Instruction::Free(r) => self.execute_free(r),
        	Instruction::Freea(a) => self.execute_freea(a),
        	Instruction::Setb(r, r1, r2) => self.execute_setb(r, r1, r2),
        	Instruction::Seti(r, r1, r2) => self.execute_seti(r, r1, r2),
        	Instruction::Isetb(i, r, r1) => self.execute_isetb(i, r, r1),
        	Instruction::Iseti(i, r, r1) => self.execute_iseti(i, r, r1),
        	Instruction::Getb(r, r1, r2) => self.execute_getb(r, r1, r2),
        	Instruction::Geti(r, r1, r2) => self.execute_geti(r, r1, r2),
        	Instruction::Igetb(i, r, r1) => self.execute_igetb(i, r, r1),
        	Instruction::Igeti(i, r, r1) => self.execute_igeti(i, r, r1),
        	Instruction::Last(r, r1) => self.execute_last(r, r1),
        	Instruction::Length(r, r1) => self.execute_length(r, r1),
        	Instruction::Clone(r, r1) => self.execute_clone(r, r1),
        	Instruction::Copy(r, r1, r2, r3, r4) => self.execute_copy(r, r1, r2, r3, r4),
        	Instruction::Copyi(i, i1, i2, r, r1) => self.execute_copyi(i, i1, i2, r, r1),
        	Instruction::Addi(r, r1, r2) => self.execute_addi(r, r1, r2),
        	Instruction::Subi(r, r1, r2) => self.execute_subi(r, r1, r2),
        	Instruction::Muli(r, r1, r2) => self.execute_muli(r, r1, r2),
        	Instruction::Divi(r, r1, r2) => self.execute_divi(r, r1, r2),
        	Instruction::Modi(r, r1, r2) => self.execute_modi(r, r1, r2),
        	Instruction::Addu(r, r1, r2) => self.execute_addu(r, r1, r2),
        	Instruction::Subu(r, r1, r2) => self.execute_subu(r, r1, r2),
        	Instruction::Mulu(r, r1, r2) => self.execute_mulu(r, r1, r2),
        	Instruction::Divu(r, r1, r2) => self.execute_divu(r, r1, r2),
        	Instruction::Modu(r, r1, r2) => self.execute_modu(r, r1, r2),
        	Instruction::Addf(r, r1, r2) => self.execute_addf(r, r1, r2),
        	Instruction::Subf(r, r1, r2) => self.execute_subf(r, r1, r2),
        	Instruction::Mulf(r, r1, r2) => self.execute_mulf(r, r1, r2),
        	Instruction::Divf(r, r1, r2) => self.execute_divf(r, r1, r2),
        	Instruction::Rotl(r, r1) => self.execute_rotl(r, r1),
        	Instruction::Rotli(i, r) => self.execute_rotli(i, r),
        	Instruction::Rotr(r, r1) => self.execute_rotr(r, r1),
        	Instruction::Rotri(i, r) => self.execute_rotri(i, r),
        	Instruction::Sll(r, r1) => self.execute_sll(r, r1),
        	Instruction::Slli(i, r) => self.execute_slli(i, r),
        	Instruction::Srl(r, r1) => self.execute_srl(r, r1),
        	Instruction::Srli(i, r) => self.execute_srli(i, r),
        	Instruction::Not(r) => self.execute_not(r),
        	Instruction::And(r, r1, r2) => self.execute_and(r, r1, r2),
        	Instruction::Or(r, r1, r2) => self.execute_or(r, r1, r2),
        	Instruction::Xor(r, r1, r2) => self.execute_xor(r, r1, r2),
        	Instruction::Cmp(r, r1) => self.execute_cmp(r, r1),
        	Instruction::Cmpi(r, r1) => self.execute_cmpi(r, r1),
        	Instruction::Cmpf(r, r1) => self.execute_cmpf(r, r1),
        	Instruction::Jmp(a) => self.execute_jmp(a),
        	Instruction::Jeq(a) => self.execute_jeq(a),
        	Instruction::Jne(a) => self.execute_jne(a),
        	Instruction::Jge(a) => self.execute_jge(a),
        	Instruction::Jgt(a) => self.execute_jgt(a),
        	Instruction::Jle(a) => self.execute_jle(a),
        	Instruction::Jlt(a) => self.execute_jlt(a),
        	Instruction::I2f(r) => self.execute_i2f(r),
        	Instruction::F2i(r) => self.execute_f2i(r),
        	Instruction::Swpa(a, a1) => self.execute_swpa(a, a1),
        	Instruction::Swpar(r, r1) => self.execute_swpar(r, r1),
        	Instruction::Swpr(r, r1) => self.execute_swpr(r, r1),
        	Instruction::Call(a) => self.execute_call(a),
        	Instruction::Ret => self.execute_ret(),
        	Instruction::Halt => self.execute_halt(),
        };
    }

    fn execute_nop(&mut self) -> Self::Output;

    fn execute_syscall<S: SyscallHandler<Self::Machine>>(&mut self, handler: &mut S, i: Immediate) -> Self::Output;

    fn execute_ldb(&mut self, i: Immediate, r: Register) -> Self::Output;

    fn execute_ldi(&mut self, i: Immediate, r: Register) -> Self::Output;

    fn execute_ldf(&mut self, i: Immediate, r: Register) -> Self::Output;

    fn execute_mov(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_push(&mut self, r: Register) -> Self::Output;

    fn execute_pop(&mut self, r: Register) -> Self::Output;

    fn execute_sget(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_malloc(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_malloci(&mut self, i: Immediate, r: Register) -> Self::Output;

    fn execute_free(&mut self, r: Register) -> Self::Output;

    fn execute_freea(&mut self, a: Address) -> Self::Output;

    fn execute_setb(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_seti(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_isetb(&mut self, i: Immediate, r: Register, r1: Register) -> Self::Output;

    fn execute_iseti(&mut self, i: Immediate, r: Register, r1: Register) -> Self::Output;

    fn execute_getb(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_geti(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_igetb(&mut self, i: Immediate, r: Register, r1: Register) -> Self::Output;

    fn execute_igeti(&mut self, i: Immediate, r: Register, r1: Register) -> Self::Output;

    fn execute_last(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_length(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_clone(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_copy(&mut self, r: Register, r1: Register, r2: Register, r3: Register, r4: Register) -> Self::Output;

    fn execute_copyi(&mut self, i: Immediate, i1: Immediate, i2: Immediate, r: Register, r1: Register) -> Self::Output;

    fn execute_addi(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_subi(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_muli(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_divi(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_modi(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_addu(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_subu(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_mulu(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_divu(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_modu(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_addf(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_subf(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_mulf(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_divf(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_rotl(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_rotli(&mut self, i: Immediate, r: Register) -> Self::Output;

    fn execute_rotr(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_rotri(&mut self, i: Immediate, r: Register) -> Self::Output;

    fn execute_sll(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_slli(&mut self, i: Immediate, r: Register) -> Self::Output;

    fn execute_srl(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_srli(&mut self, i: Immediate, r: Register) -> Self::Output;

    fn execute_not(&mut self, r: Register) -> Self::Output;

    fn execute_and(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_or(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_xor(&mut self, r: Register, r1: Register, r2: Register) -> Self::Output;

    fn execute_cmp(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_cmpi(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_cmpf(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_jmp(&mut self, a: Address) -> Self::Output;

    fn execute_jeq(&mut self, a: Address) -> Self::Output;

    fn execute_jne(&mut self, a: Address) -> Self::Output;

    fn execute_jge(&mut self, a: Address) -> Self::Output;

    fn execute_jgt(&mut self, a: Address) -> Self::Output;

    fn execute_jle(&mut self, a: Address) -> Self::Output;

    fn execute_jlt(&mut self, a: Address) -> Self::Output;

    fn execute_i2f(&mut self, r: Register) -> Self::Output;

    fn execute_f2i(&mut self, r: Register) -> Self::Output;

    fn execute_swpa(&mut self, a: Address, a1: Address) -> Self::Output;

    fn execute_swpar(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_swpr(&mut self, r: Register, r1: Register) -> Self::Output;

    fn execute_call(&mut self, a: Address) -> Self::Output;

    fn execute_ret(&mut self) -> Self::Output;

    fn execute_halt(&mut self) -> Self::Output;

}

