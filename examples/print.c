#include <unistd.h>

int main() {
    write(2, "hi", 2);
    return 0;
}
