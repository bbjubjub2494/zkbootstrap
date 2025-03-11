#define ECALL_SOFTWARE 2
#define SYS_READ "risc0_zkvm_platform::syscall::nr::SYS_READ"
#define SYS_WRITE "risc0_zkvm_platform::syscall::nr::SYS_WRITE"

int syscall3(int ecall_type, void *dst, int dst_len_in_words, int, int, int) {
		asm(
	    "rd_t0 rs1_sp !20 lw"
	    "rd_a0 rs1_sp !16 lw"
	    "rd_a1 rs1_sp !12 lw"
	    "rd_a2 rs1_sp !8 lw"
	    "rd_a3 rs1_sp !4 lw"
	    "rd_a4 rs1_sp !0 lw"
    "ecall"                 // ecall(t0, a0, a1, a2, a3, a4)
	    );
}

int syscall4(int ecall_type, void *dst, int dst_len_in_words, int, int, int, int) {
		asm(
	    "rd_t0 rs1_sp !24 lw"
	    "rd_a0 rs1_sp !20 lw"
	    "rd_a1 rs1_sp !16 lw"
	    "rd_a2 rs1_sp !12 lw"
	    "rd_a3 rs1_sp !8 lw"
	    "rd_a4 rs1_sp !4 lw"
	    "rd_a5 rs1_sp !0 lw"
    "ecall"                 // ecall(t0, a0, a1, a2, a3, a4)
	    );
}

int write(int fd, char *data, unsigned nbytes) {
	return syscall4(ECALL_SOFTWARE, 0, 0, SYS_WRITE, fd, data, nbytes);
}
int read(int fd, char *buf, unsigned nbytes) {
    return syscall3(ECALL_SOFTWARE, buf, (nbytes + 3)/4, SYS_READ, fd, nbytes);
}


int main() {
	write(1, "Hello world!", 12);
}
