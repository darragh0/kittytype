#include "ui.h"
#include "kb.h"


int main() {
    setup_ui();
    mainloop();
    cleanup_ui();

    return 0;
}
