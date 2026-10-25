#include <stdbool.h>

bool check_ventura() {
    if (@available(macOS 13.0, *)) return true;
    return false;
}