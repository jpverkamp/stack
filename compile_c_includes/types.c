#define TAG_NUMBER 0
#define TAG_NUMBER_INTEGER 1
#define TAG_NUMBER_RATIONAL 2
#define TAG_NUMBER_FLOAT 3
#define TAG_NUMBER_COMPLEX 4

#define TAG_STRING 16
#define TAG_BOOLEAN 17
#define TAG_BLOCK 18

#define TAG_STACK 32

// Values on the stack
typedef struct
{
    uint8_t type;
    union
    {
        int64_t as_integer;
        double as_float;

        char *as_string;
        bool as_boolean;
        void *as_block;

        void *as_stack;
    };
} Value;

/* ----- ----- ----- */

// A dynamically resized stack of values
typedef struct
{
    size_t capacity;
    size_t size;
    Value *values;

} ValueStack;

#define VS_INITIAL_CAPACITY 8

// Initialize the stack with default capacity
ValueStack *vs_init()
{
    ValueStack *stack = malloc(sizeof(ValueStack));
    Value *values = malloc(sizeof(Value) * VS_INITIAL_CAPACITY);

    if (!stack || !values)
    {
        fprintf(stderr, "Failed to allocate memory for a ValueStack\n");
        exit(1);
    }

    stack->capacity = VS_INITIAL_CAPACITY;
    stack->size = 0;
    stack->values = values;

    return stack;
}

// Ensure there's room to push another value onto the stack
void vs_ensure_capacity(ValueStack *stack, size_t new_size)
{
    if (new_size >= stack->capacity)
    {
        size_t new_capacity = stack->capacity * 2;
        Value *new_values = realloc(stack->values, sizeof(Value) * new_capacity);

        if (!new_values)
        {
            fprintf(stderr, "Failed to re-allocate memory for a ValueStack\n");
            exit(1);
        }

        stack->capacity = new_capacity;
        stack->values = new_values;
    }
}

// Push a value onto the stack
void vs_push(ValueStack *stack, Value val)
{
    vs_ensure_capacity(stack, stack->size + 1);
    stack->values[stack->size++] = val;
}

// Pop and return a  value from the stack
Value *vs_pop(ValueStack *stack)
{
    if (stack->size == 0)
    {
        fprintf(stderr, "Attempted to pop from an empty stack\n");
        exit(1);
    }

    return &stack->values[--stack->size];
}

// Get a value from the stack by index without removing it
Value *vs_get(ValueStack *stack, size_t index)
{
    if (index >= stack->size)
    {
        fprintf(stderr, "Attempted to get a value from the stack at an invalid index (%lu, size is %lu)\n", index, stack->size);
        exit(1);
    }

    return &stack->values[index];
}

// Set a value in the stack at a given index
void vs_set(ValueStack *stack, size_t index, Value value)
{
    if (index >= stack->size)
    {
        fprintf(stderr, "Attempted to get a value from the stack at an invalid index (%lu, size is %lu)\n", index, stack->size);
        exit(1);
    }

    stack->values[index] = value;
}

/* ----- */

void value_write(FILE *f, Value *v)
{
    if (v->type == TAG_NUMBER_INTEGER)
    {
        fprintf(f, "%lld", v->as_integer);
    }
    else if (v->type == TAG_NUMBER_FLOAT)
    {
        fprintf(f, "%f", v->as_float);
    }
    else if (v->type == TAG_STRING)
    {
        fprintf(f, "%s", v->as_string);
    }
    else if (v->type == TAG_BOOLEAN)
    {
        fprintf(f, "%s", v->as_boolean ? "true" : "false");
    }
    else if (v->type == TAG_BLOCK)
    {
        fprintf(f, "{block}");
    }
    else if (v->type == TAG_STACK)
    {
        fprintf(f, "[");
        ValueStack *stack = v->as_stack;
        Value *ptr = stack->values;

        for (int i = 0; i < stack->size; i++)
        {
            value_write(f, ptr++);

            if (i != stack->size - 1)
            {
                fprintf(f, ", ");
            }
        }
        fprintf(f, "]");
    }
    else
    {
        fprintf(stderr, "Error: unknown type on stack: %u\n", v->type);
        exit(1);
    }
}
