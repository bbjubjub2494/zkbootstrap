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
    "ecall"                 // ecall(t0, a0, a1, a2, a3, a4, a5)
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

int write(int fd, char *data, unsigned nbytes) {
	return syscall4(ECALL_SOFTWARE, 0, 0, SYS_WRITE, fd, data, nbytes);
}
int read(int fd, char *buf, unsigned nbytes) {
	// TODO handle weird last word behaviour
    return syscall4(ECALL_SOFTWARE, buf, (nbytes + 3)/4, SYS_READ, fd, buf, nbytes);
}

void sha_compress(unsigned *sha_state, unsigned *blocks, unsigned block_count) {
    syscall3(
        ECALL_SHA,
        sha_state,
        sha_state,
        blocks,
        blocks + DIGEST_BYTES,
        block_count
    );
}

unsigned htonl(unsigned x) {
	return ((x & 0xff) << 24) | (((x >> 8) & 0xff) << 16) | (((x >> 16) & 0xff) << 8) | ((x >> 24) & 0xff);
}

void sha_reset(unsigned *sha_state) {
	sha_state[0] = htonl(0x6a09e667);
	sha_state[1] = htonl(0xbb67ae85);
	sha_state[2] = htonl(0x3c6ef372);
	sha_state[3] = htonl(0xa54ff53a);
	sha_state[4] = htonl(0x510e527f);
	sha_state[5] = htonl(0x9b05688c);
	sha_state[6] = htonl(0x1f83d9ab);
	sha_state[7] = htonl(0x5be0cd19);
}

void sha_finalize(unsigned *sha_state, char *last_block, unsigned total_bytes) {
	// requirement: sha_state must be a sha256 internal state
	// requirement: last_block must be aligned and 32-byte
	// requirement: there must be a 32-byte zeroed scratch space behind the last block
	unsigned bitlenpos = 56;
	unsigned block_count = 1;
	if ((total_bytes % 64) >= bitlenpos) {
		bitlenpos += BLOCK_BYTES;
		block_count += 1;
	}
	last_block[total_bytes % 64] = 0x80;
	unsigned len = total_bytes << 3;
	last_block[bitlenpos+4] = len >> 24;
	last_block[bitlenpos+5] = len >> 16;
	last_block[bitlenpos+6] = len >> 8;
	last_block[bitlenpos+7] = len;
	sha_compress(sha_state, last_block, block_count);
}

void halt(unsigned user_exit, unsigned *out_state) {
    ecall1(
        SYS_HALT,
        HALT_TERMINATE | (user_exit << 8),
        out_state
    );

    // unreachable
}


unsigned sha_state_stdin[DIGEST_WORDS];
unsigned sha_state_stdout[DIGEST_WORDS];
unsigned sha_state_journal[DIGEST_WORDS];
unsigned sha_state_rzoutput[DIGEST_WORDS];

// 1 block working space, 1 extra block to deal with padding
char buf_stdin[128];
char buf_stdout[128];
char buf[128];

// FIXME something weird with the registers breaks stack variables
unsigned len;
unsigned total_bytes;
unsigned nbytes;

void *memcpy(char *dest, char *src, unsigned n) {
	unsigned i = 0;
	while (i < n) {
		dest[i] = src[i];
		i+=1;
	}
	return dest;
}

void *memzero(char *buf, unsigned n) {
	unsigned i = 0;
	while (i < n) {
		buf[i] = 0;
		i+=1;
	}
	return buf;
}

int main() {
	sha_reset(sha_state_stdin);
	sha_reset(sha_state_stdout);
	while(1) {
		nbytes = read(0, buf_stdin, BLOCK_BYTES);
		write(1, buf_stdin, nbytes);
		total_bytes += nbytes;
		if (nbytes == BLOCK_BYTES) {
			sha_compress(sha_state_stdin, buf_stdin, 1);
			sha_compress(sha_state_stdout, buf_stdin, 1);
		} else {
			sha_finalize(sha_state_stdin, buf_stdin, total_bytes);
			sha_finalize(sha_state_stdout, buf_stdin, total_bytes);
			break;
		}
	}

	write(3, sha_state_stdin, DIGEST_BYTES);
	write(3, sha_state_stdout, DIGEST_BYTES);
	sha_reset(sha_state_journal);
	memzero(buf, 128);
	memcpy(buf, sha_state_stdin, DIGEST_BYTES);
	memcpy(buf + DIGEST_BYTES, sha_state_stdout, DIGEST_BYTES);
	sha_compress(sha_state_journal, buf, 1);
	memzero(buf, 128);
	sha_finalize(sha_state_journal, buf, DIGEST_BYTES*2);

	sha_reset(sha_state_rzoutput);
	memzero(buf, 128);
	memcpy(buf, "w\xea\xfe\xb3f\xa7\x8bGt}\xe0\xd7\xbb\x17b\x84\x08_\xf5VH\x87\x00\x9a[\xe6=\xa3-5Y\xd4", DIGEST_BYTES); // sha2("risc0.Output")
	memcpy(buf + DIGEST_BYTES, sha_state_journal, DIGEST_BYTES);
	sha_compress(sha_state_rzoutput, buf, 1);
	memzero(buf, 128);
	// 32 zero bytes for empty assumption logs
	buf[DIGEST_BYTES] = 2; // end with little-endian 16-bit 2
	sha_finalize(sha_state_rzoutput, buf, DIGEST_BYTES*3 + 2);

	halt(0, sha_state_rzoutput);
}
