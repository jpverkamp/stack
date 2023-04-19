
// Lookup a value on the stack by name
Value *lookup(Value *stack, Value *stack_ptr, uint8_t name)
{
    Value *ptr = stack_ptr;
    while (ptr >= stack)
    {
        for (int i = 0; i < ptr->name_count; i++)
        {
            if (ptr->names[i] == name)
            {
                return ptr;
            }
        }
        ptr--;
    }

    printf("Name not found: %d", name);
    exit(1);
}