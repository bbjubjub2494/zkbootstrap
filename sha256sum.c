#define SYS_HALT 0
#define ECALL_SOFTWARE 2
#define ECALL_SHA 3

#define SYS_READ "risc0_zkvm_platform::syscall::nr::SYS_READ"
#define SYS_WRITE "risc0_zkvm_platform::syscall::nr::SYS_WRITE"

#define HALT_TERMINATE 0

#define DIGEST_WORDS 8
#define DIGEST_BYTES 32
#define BLOCK_BYTES 64


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

/*
// NOTE: unpadded, needs proper in_state
# define MAX_SHA_COMPRESS_BLOCKS 1000
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
*/

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


unsigned sha_state[DIGEST_WORDS];

// 1 block working space, 1 extra block to deal with padding
char buf[128];
unsigned len = 0;
	unsigned bitlen;
	unsigned bitlenpos;
	unsigned count;
	unsigned i;
	unsigned totallen;

unsigned htonl(unsigned x) {
	return ((x & 0xff) << 24) | (((x >> 8) & 0xff) << 16) | (((x >> 16) & 0xff) << 8) | ((x >> 24) & 0xff);
}
int main() {
	sha_state[0] = htonl(0x6a09e667);
	sha_state[1] = htonl(0xbb67ae85);
	sha_state[2] = htonl(0x3c6ef372);
	sha_state[3] = htonl(0xa54ff53a);
	sha_state[4] = htonl(0x510e527f);
	sha_state[5] = htonl(0x9b05688c);
	sha_state[6] = htonl(0x1f83d9ab);
	sha_state[7] = htonl(0x5be0cd19);
	while(1) {
		len = read(0, buf, BLOCK_BYTES);
		totallen += len;
		if (len == BLOCK_BYTES) {
			sys_sha_compress(sha_state, sha_state, buf, buf + DIGEST_BYTES, 1);
		} else {
			bitlenpos = 56;
			count = 1;
			if (len >= bitlenpos) {
				bitlenpos += BLOCK_BYTES;
				count += 1;
			}
			buf[len] = 0x80;
			bitlen = totallen << 3;
			//buf[bitlenpos] = bitlen >> 56;
			//buf[bitlenpos+1] = bitlen >> 48;
			//buf[bitlenpos+2] = bitlen >> 40;
			//buf[bitlenpos+3] = bitlen >> 32;
			buf[bitlenpos+4] = bitlen >> 24;
			buf[bitlenpos+5] = bitlen >> 16;
			buf[bitlenpos+6] = bitlen >> 8;
			buf[bitlenpos+7] = bitlen;
			sys_sha_compress(sha_state, sha_state, buf, buf + DIGEST_BYTES, count);
			break;
		}
	}
	write(1, sha_state, DIGEST_BYTES);
}
