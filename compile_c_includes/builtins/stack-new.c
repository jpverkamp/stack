{
    Value v = {.type = TAG_STACK, .as_stack = vs_init()};
    *(++stack_ptr) = v;
}
