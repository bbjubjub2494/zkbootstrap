int c; // FIXME stack variables in main() broken for some reason
int main() {
	j_prepare();

	while ((c = getchar()) >= 0)
		putchar(c);

	j_finalize_and_halt();
}
