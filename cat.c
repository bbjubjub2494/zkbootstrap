#define SYS_HALT 0
#define ECALL_SOFTWARE 2
#define ECALL_SHA 3

#define SYS_READ "risc0_zkvm_platform::syscall::nr::SYS_READ"
#define SYS_WRITE "risc0_zkvm_platform::syscall::nr::SYS_WRITE"

#define HALT_TERMINATE 0

#define DIGEST_WORDS 8
#define DIGEST_BYTES 32
#define MAX_SHA_COMPRESS_BLOCKS 1000

int write(int fd, char *data, unsigned nbytes);

void print(unsigned x) {
	char d;
	while (x > 0) {
	if ((x % 16) < 10) {
		d = (x % 10) + '0';
	} else {
		d = (x % 16) - 10 + 'a';
	}
	write(1, &d, 1);
	x >>= 4;
	}
	write(1, "\n", 1);
}


int syscall3(int ecall_type, void *dst, int dst_len_in_words, int, int, int) {
		asm(
	    "rd_t0 rs1_sp !24 lw"
	    "rd_a0 rs1_sp !20 lw"
	    "rd_a1 rs1_sp !16 lw"
	    "rd_a2 rs1_sp !12 lw"
	    "rd_a3 rs1_sp !8 lw"
	    "rd_a4 rs1_sp !4 lw"
    "ecall"                 // ecall(t0, a0, a1, a2, a3, a4)
	    "rs1_sp rs2_a0 !0 sw"
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
    "ecall"                 // ecall(t0, a0, a1, a2, a3, a4,a5)
	    );
}

int ecall1(int, int, int) {
		asm(
	    "rd_t0 rs1_sp !8 lw"
	    "rd_a0 rs1_sp !4 lw"
	    "rd_a1 rs1_sp !0 lw"
	    "ecall"                 // ecall(t0, a0, a1)
	    );
}

void sys_sha_compress(unsigned *out_state, unsigned *in_state, unsigned *block1_ptr, unsigned *block2_ptr, unsigned count) {
    syscall3(
        ECALL_SHA,
        out_state,
        in_state,
        block1_ptr,
        block2_ptr,
        count
    );
}


unsigned min(unsigned a, unsigned b) {
    if (a < b)
	    return a;
    else
	    return b;
}

// NOTE: unpadded, needs proper in_state
void sys_sha_buffer(unsigned *out_state, unsigned *in_state, char *buf, unsigned count) {
    char *ptr = buf;
    unsigned count_remain = count;
    unsigned *current_in_state = in_state;

    while (count_remain > 0) {
        count = min(count_remain, MAX_SHA_COMPRESS_BLOCKS);
        sys_sha_compress(
            out_state,
            current_in_state,
            ptr,
            ptr + DIGEST_BYTES,
            count
        );

        count_remain -= count;
        ptr += 2 * DIGEST_BYTES * count;
        current_in_state = out_state;
    }
}

void sys_halt(unsigned user_exit, unsigned *out_state) {
    ecall1(
        SYS_HALT,
        HALT_TERMINATE | (user_exit << 8),
        out_state
    );

    // unreachable
}

int write(int fd, char *data, unsigned nbytes) {
	return syscall4(ECALL_SOFTWARE, 0, 0, SYS_WRITE, fd, data, nbytes);
}
int read(int fd, char *buf, unsigned nbytes) {
	// TODO handle weird last word behaviour
		return syscall4(ECALL_SOFTWARE, buf, (nbytes + 3)/4, SYS_READ, fd, buf, nbytes);
}


unsigned in_state[DIGEST_WORDS];
unsigned out_state[DIGEST_WORDS];

#define BUF_SIZE 1024
char buf[BUF_SIZE];

	unsigned nbytes;

int main() {
	nbytes = read(0, buf, BUF_SIZE);
	write(1, buf, nbytes);
}
