int main(int argc, char *argv[])
{
    // The stack holding all values
    stack = malloc(10240 * sizeof(Value));
    stack_ptr = stack;

    // Frames holding the stack pointer for each block
    frames = malloc(10240 * sizeof(Value **));
    frame_ptr = frames;

    block_0(NULL);

    return 0;
}