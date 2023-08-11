// Names linked list
typedef struct Name Name;
struct Name
{
    bool boundary;
    uint8_t name;
    Value *value;
    Name *prev;
};

Name *names_bind(Name *names, uint8_t name, Value *value)
{
    Name *new_name = malloc(sizeof(Name));
    if (new_name == NULL)
    {
        fprintf(stderr, "Out of memory");
        exit(1);
    }

    new_name->boundary = false;
    new_name->name = name;
    new_name->value = value;
    new_name->prev = names;
    return new_name;
}

// Lookup a value on the stack by name
Value *names_lookup(Name *names, uint8_t name)
{
    while (names != NULL)
    {
        if (names->name == name)
        {
            return names->value;
        }
        names = names->prev;
    }

    fprintf(stderr, "Error in names_lookup(); name not found: %d", name);
    exit(1);
}

// Update a value on the stack by name
void names_update(Name *names, uint8_t name, Value *value)
{
    while (names != NULL)
    {
        if (names->name == name)
        {
            names->value = value;
            return;
        }
        names = names->prev;
    }

    fprintf(stderr, "Error in names_update(); name not found: %d", name);
    exit(1);
}