int main(int argc, char *argv[])
{
    // The stack holding all values
    stack = malloc(1024 * sizeof(Value));
    stack_ptr = stack;

    // Frames holding the stack pointer for each block
    frames = malloc(1024 * sizeof(Value **));
    frame_ptr = frames;

    block_0();

    return 0;
}