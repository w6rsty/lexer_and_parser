typedef struct {
    int a;
    char b;
} my_struct;

enum my_enum {
    eA,
    eB,
    eC = 5,
    eD,
};

int main() {
    // This is a single-line comment
    /* This is a
        multi-line comment */
    int expr = 5 / 2;
    if (expr == 2) {
        return 0;
    }

    char ch = 'c';
    char *s = "string";

    int integers[] = { 42, 123U, 456u, 789L, 101112l, 131415UL, 161718lu, 192021LL, 222324ll, 252627ULL, 282930llu, 0xFFu, 0777L };

    float floats[] = { 3.14, 1.23F, 4.56f, 7.89L, };

    ((int*)&floats[0] + 1u)[12] = 12;
}

