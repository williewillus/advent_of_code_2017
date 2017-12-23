#include <stdint.h>
#include <inttypes.h>
#include <stdio.h>

int64_t a = 1;
int64_t b = 0;
int64_t c = 0;
int64_t d = 0;
int64_t e = 0;
int64_t f = 0;
int64_t g = 0;
int64_t h = 0;

int main() {
	b = 57;
	c = b;
	if (a != 0) {
		b *= 100;
		b -= -100000;
		c = b;
		c -= -17000;
	}

	printf("b start %" PRId64 "\n", b);
	
	L_I:
	f = 1;
	d = 2;
	do {
		e = 2;

		if(b % d == 0 && b/d != 1) {
			f = 0;
		}
	
		// do {
		// 	if (d*e == b)
		// 		f = 0;
		// 	e += 1;
		// } while (e != b);

		d += 1;		
	} while (d != b);
	
	if (f == 0) {
		h += 1;
		printf("a:%" PRId64 "\n", a);
		printf("b:%" PRId64 "\n", b);
		printf("c:%" PRId64 "\n", c);
		printf("d:%" PRId64 "\n", d);
		printf("e:%" PRId64 "\n", e);
		//printf("%" PRId64 "\n", f);
		//printf("%" PRId64 "\n", g);
		printf("h:%" PRId64 "\n", h);
	}
	
	if (b-c == 0) {
		printf("part 2: %" PRId64 "\n", h);
		return 0;		
	}
	b += 17;
	goto L_I;
}

// we terminate when b - c is 0
// c appears to always be 122700
// when h first goes 0->1 b is 105700
// b,d,e appear to increase by 17 each time h is bumped
// so h should be 122700 - 105700, / 17
