// Names linked list
typedef struct Name Name;
struct Name
{
    bool boundary;
    uint8_t name;
    Value *value;
    Name *prev;
};

Name *bind(Name *names, uint8_t name, Value *value)
{
    Name *new_name = malloc(sizeof(Name));
    if (new_name == NULL)
    {
        printf("Out of memory");
        exit(1);
    }

    new_name->boundary = false;
    new_name->name = name;
    new_name->value = value;
    new_name->prev = names;
    return new_name;
}

// Lookup a value on the stack by name
Value *lookup(Name *names, uint8_t name)
{
    while (names != NULL)
    {
        if (names->name == name)
        {
            return names->value;
        }
        names = names->prev;
    }

    printf("Name not found: %d", name);
    exit(1);
}