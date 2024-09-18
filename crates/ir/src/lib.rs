type Register = u64;
type Immediate = u64;

#[derive(Debug)]
enum IR {
  Add(Register, Register, Register),
  Addi(Register, Register, Immediate),
  Ecall,
}
