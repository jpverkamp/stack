#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// #region Generated debug flag
/*{DEBUG}*/
// #endregion

// #region Values on the stack
#define TAG_NUMBER 0
#define TAG_NUMBER_INTEGER 1
#define TAG_NUMBER_RATIONAL 2
#define TAG_NUMBER_FLOAT 3
#define TAG_NUMBER_COMPLEX 4

#define TAG_STRING 16
#define TAG_BOOLEAN 17
#define TAG_BLOCK 18

#define TAG_STACK 32

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
// #endregion

// #region A dynamically sized vector/stack of Values
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
// #endregion

// #region Global data structures
// The stack holding all values
Value *stack;
Value *stack_ptr;

// Frames holding the stack pointer for each block
Value **frames;
Value **frame_ptr;
// #endregion

// #region Generated name constants
/*{NAMES}*/
// #endregion

// #region Data and functions for naming variables on the stack
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
            *names->value = *value;
            return;
        }
        names = names->prev;
    }

    fprintf(stderr, "Error in names_update(); name not found: %d", name);
    exit(1);
}
// #endregion

// #region Printing values or the stack
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

void stack_dump(Name *names)
{
    if (stack_ptr == stack)
    {
        fprintf(stderr, " STACK: <empty> ");
    }
    else
    {
        fprintf(stderr, " STACK: ");
        for (Value *ptr = stack_ptr; ptr != stack; ptr--)
        {
            value_write(stderr, ptr);
            fprintf(stderr, " ");
        }
    }

    if (names != NULL)
    {
        fprintf(stderr, "NAMES: ");
        while (names != NULL)
        {
            fprintf(stderr, "%s=", get_name(names->name));
            value_write(stderr, names->value);

            if (names->boundary && names->prev != NULL)
            {
                fprintf(stderr, " | ");
            }
            else
            {
                fprintf(stderr, " ");
            }

            names = names->prev;
        }
    }

    fprintf(stderr, "\n");
}
// #endregion

// #region Assertions that print an error and exit if they fail
void assert_type(char *name, char *type_name, uint8_t type_tag, Value *value, Name *names)
{
    if (value->type != type_tag)
    {
        fprintf(stderr, "Error in %s, expected a %s, got: ", name, type_name);
        value_write(stderr, value);
        fprintf(stderr, " with");
        stack_dump(names);
        exit(1);
    }
}
// #endregion

// #region Functions for converting between types
// Convert two values to have the same type by upgrading if necessary
void coerce(Value *a, Value *b)
{
    if (a->type == b->type)
    {
        return;
    }

    if (a->type == TAG_NUMBER_INTEGER && b->type == TAG_NUMBER_FLOAT)
    {
        a->type = TAG_NUMBER_FLOAT;
        a->as_float = (double)a->as_integer;
    }

    if (a->type == TAG_NUMBER_FLOAT && b->type == TAG_NUMBER_INTEGER)
    {
        b->type = TAG_NUMBER_FLOAT;
        b->as_float = (double)b->as_integer;
    }
}
// #endregion

// #region Generated block definitions
/*{BLOCKS}*/
// #endregion

// #region The main function
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
// #endregion