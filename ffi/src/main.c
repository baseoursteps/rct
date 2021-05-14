#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

extern size_t **
matrix(size_t N);

extern void
free_matrix(size_t **m, size_t sz);

int
main()
{
    const size_t N = 4;
    size_t **    m = matrix(N);

    for (size_t i = 0; i < N; ++i) {
        for (size_t j = 0; j < N; ++j)

            printf(" %zu ", m[i][j]);

        puts("");
    }

    free_matrix(m, N);
}
