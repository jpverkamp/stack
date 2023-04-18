
int main(int argc, char *argv[])
{
    // The stack holding all values
    Value *stack = malloc(1024 * sizeof(Value));
    Value *stack_ptr = stack;

    Value **frames = malloc(1024 * sizeof(Value **));
    Value **frame_ptr = frames;
