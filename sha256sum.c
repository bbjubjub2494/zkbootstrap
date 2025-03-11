#define SYS_HALT 0
#define ECALL_SOFTWARE 2
#define ECALL_SHA 3

#define SYS_READ "risc0_zkvm_platform::syscall::nr::SYS_READ"
#define SYS_WRITE "risc0_zkvm_platform::syscall::nr::SYS_WRITE"

#define HALT_TERMINATE 0

#define DIGEST_WORDS 8
#define DIGEST_BYTES 32
#define MAX_SHA_COMPRESS_BLOCKS 1000


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
		// TODO handle return values in a0, a1
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
    return syscall3(ECALL_SOFTWARE, buf, (nbytes + 3)/4, SYS_READ, fd, nbytes);
}


unsigned in_state[DIGEST_WORDS];
unsigned out_state[DIGEST_WORDS];

char buf[64];
unsigned len = 0;

unsigned htonl(unsigned x) {
	return ((x & 0xff) << 24) | (((x >> 8) & 0xff) << 16) | (((x >> 16) & 0xff) << 8) | ((x >> 24) & 0xff);
}
int main() {
	in_state[0] = htonl(0x6a09e667);
	in_state[1] = htonl(0xbb67ae85);
	in_state[2] = htonl(0x3c6ef372);
	in_state[3] = htonl(0xa54ff53a);
	in_state[4] = htonl(0x510e527f);
	in_state[5] = htonl(0x9b05688c);
	in_state[6] = htonl(0x1f83d9ab);
	in_state[7] = htonl(0x5be0cd19);
	// padding: 1 byte marker, 8-byte 64-bit length and zeroes in-between
	buf[len] = 0x80;
	//char* last_block = &buf[(len + 9) / 32 * 32];
	/*
	unsigned i;
	for (i = 0; i < 8; i+=1)
		buf[32-i] = len >> (8*i);
		*/

	sys_sha_buffer(out_state, in_state, buf, 1);
	// note: endianness
	write(1, out_state, DIGEST_BYTES);
	//write(1, in_state, 32);
}
