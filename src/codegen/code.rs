pub const PREPARE: &str = r#"#include <stdio.h>
#include <stdlib.h>

typedef unsigned long IdxType;

#define YYTEXT_MAXLEN 1024

FILE *yyin = NULL, *yyout = NULL;

char *g_buffer;
char *g_buffer_ptr;
IdxType g_buflen;
IdxType g_bufidx;

char yytext[YYTEXT_MAXLEN];
IdxType yyleng;

void yy_read_buffer() {
  fseek(yyin, 0, SEEK_END);
  g_buflen = ftell(yyin);
  rewind(yyin);

  g_buffer = malloc(g_buflen + 1);
  if (!g_buffer) {
    perror("Failed to allocate memory for buffer");
  }

  fread(g_buffer, 1, g_buflen, yyin);
  g_buffer[g_buflen] = '\0';
  fclose(yyin);
}

"#;

pub const REGEX: &str = r#"
/*
 * Merged regex library
 * For embedding into lers projects
 */

#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

/*
 * ============================================================================
 * state.c - State management
 * ============================================================================
 */

#define MAX 999

typedef unsigned short State;

typedef struct States {
  State states[MAX];
  size_t len;
} States;

/* create an empty container of states */
States *new_states() {
  States *s = (States *)malloc(sizeof(States));
  s->len = 0;
  return s;
}

/* push a state into the container */
void push_state(States *s, State state) {
  s->states[s->len] = state;
  ++(s->len);
}

/* if the states is empty */
bool states_is_empty(States *s) { return s->len == 0; }

/* if the container has the state */
bool have_state(States *s, State state) {
  for (size_t i = 0; i < s->len; ++i)
    if (s->states[i] == state)
      return true;
  return false;
}

/* if two containers have the same state, return the state */
State get_shared_states(States *s1, States *s2) {
  for (size_t i = 0; i < s1->len; ++i) {
    for (size_t j = 0; j < s2->len; ++j) {
      if (s1->states[i] == s2->states[j])
        return s1->states[i];
    }
  }
  return 0;
}

/* print states of the container */
void print_states(States *s) {
  printf("States[");
  if (s->len == 0) {
    printf("]\n");
  } else if (s->len == 1) {
    printf("%hu]\n", s->states[0]);
  } else {
    for (size_t i = 0; i < s->len - 1; ++i) {
      printf("%hu, ", s->states[i]);
    }
    printf("%hu]\n", s->states[s->len - 1]);
  }
}

/*
 * ============================================================================
 * builder/lexer.c - Lexer for regular expressions
 * ============================================================================
 */

/*
 * Lexer
 */

typedef enum TokenType {
  LPAREN,
  RPAREN,
  LBRACKET,
  RBRACKET,
  CARET,
  DASH,
  DOT,
  PLUS,
  ASTERISK,
  BAR,
  BACK_SLASH,
  LITERAL,
  END
} TokenType;

typedef struct Token {
  TokenType type;
  char value; /* valid only for LITERAL type */
} Token;

/* create a token */
Token *new_token(TokenType type, char value) {
  Token *token = (Token *)malloc(sizeof(Token));
  token->type = type;
  token->value = value;
  return token;
}

typedef struct Lexer {
  char *pattern;
  char *current_char;
  Token *current_token;
} Lexer;

/* create a new lexer from pattern string */
Lexer *new_lexer(char *pattern) {
  Lexer *lexer = (Lexer *)malloc(sizeof(Lexer));
  lexer->pattern = pattern;
  lexer->current_char = lexer->pattern;
  lexer->current_token = NULL;
  return lexer;
}

Token *get_next_token(Lexer *lexer) {
  if (lexer->current_token != NULL)
    free(lexer->current_token);

  char current_char = *lexer->current_char;
  switch (current_char) {
  case '\0':
    lexer->current_token = new_token(END, '\0');
    break;
  case '(':
    lexer->current_token = new_token(LPAREN, '(');
    break;
  case ')':
    lexer->current_token = new_token(RPAREN, ')');
    break;
  case '[':
    lexer->current_token = new_token(LBRACKET, '[');
    break;
  case ']':
    lexer->current_token = new_token(RBRACKET, ']');
    break;
  case '^':
    lexer->current_token = new_token(CARET, '^');
    break;
  case '-':
    lexer->current_token = new_token(DASH, '-');
    break;
  case '.':
    lexer->current_token = new_token(DOT, '.');
    break;
  case '+':
    lexer->current_token = new_token(PLUS, '+');
    break;
  case '*':
    lexer->current_token = new_token(ASTERISK, '*');
    break;
  case '|':
    lexer->current_token = new_token(BAR, '|');
    break;
  case '\\':
    lexer->current_token = new_token(BACK_SLASH, '\\');
    break;
  default:
    lexer->current_token = new_token(LITERAL, current_char);
    break;
  }
  ++(lexer->current_char);

  return lexer->current_token;
}

/*
 * ============================================================================
 * builder/ast.c - Abstract Syntax Tree
 * ============================================================================
 */

typedef enum AstType {
  LiteralNode,
  SetNode,
  AndNode,
  OrNode,
  RepeatNode,
  SurroundNode,
} AstType;

typedef struct Ast Ast;
typedef struct Vector_char Vector_char;

typedef struct Ast {
  enum { AstLiteral, AstSet, AstAnd, AstOr, AstRepeat, AstSurround } type;

  union {
    struct AstLiteral {
      char value;
    } AstLiteral;

    struct AstSet {
      Vector_char *set;
      bool is_neg;
    } AstSet;

    struct AstAnd {
      Ast *r1;
      Ast *r2;
    } AstAnd;

    struct AstOr {
      Ast *r1;
      Ast *r2;
    } AstOr;

    struct AstRepeat {
      Ast *r;
    } AstRepeat;

    struct AstSurround {
      Ast *r;
    } AstSurround;
  } data;
} Ast;

Ast *new_ast(Ast ast) {
  Ast *p = malloc(sizeof(Ast));
  if (p != NULL)
    *p = ast;
  return p;
}

#define NEW_AST(tag, ...)                                                      \
  new_ast((Ast){tag, {.tag = (struct tag){__VA_ARGS__}}})

/* newer */
static Ast *new_ast_literal(char value) { return NEW_AST(AstLiteral, value); }

static Ast *new_ast_set(Vector_char *set, bool is_neg) {
  return NEW_AST(AstSet, set, is_neg);
}

static Ast *new_ast_and(Ast *r1, Ast *r2) { return NEW_AST(AstAnd, r1, r2); }

static Ast *new_ast_or(Ast *r1, Ast *r2) { return NEW_AST(AstOr, r1, r2); }

static Ast *new_ast_repeat(Ast *r) { return NEW_AST(AstRepeat, r); }

static Ast *new_ast_surround(Ast *r) { return NEW_AST(AstSurround, r); }

/* clone */
static Ast *clone_ast(Ast *r) {
  if (r == NULL)
    return NULL;
  switch (r->type) {
  case LiteralNode:
    return new_ast_literal(r->data.AstLiteral.value);
  case SetNode:
    return new_ast_set(r->data.AstSet.set, r->data.AstSet.is_neg);
  case AndNode:
    return new_ast_and(clone_ast(r->data.AstAnd.r1),
                       clone_ast(r->data.AstAnd.r2));
  case OrNode:
    return new_ast_or(clone_ast(r->data.AstOr.r1), clone_ast(r->data.AstOr.r2));
  case RepeatNode:
    return new_ast_repeat(clone_ast(r->data.AstRepeat.r));
  case SurroundNode:
    return clone_ast(clone_ast(r->data.AstSurround.r));
  }
  return NULL; /* unreachable */
}

/* comparison */
bool equal_ast(Ast *a, Ast *b) {
  if (a == b)
    return true;
  if (a == NULL || b == NULL)
    return false;
  if (a->type != b->type)
    return false;

  switch (a->type) {
  case LiteralNode:
    return a->data.AstLiteral.value == b->data.AstLiteral.value;
  case SetNode:
    return true; /* ignore it */
  case AndNode:
    return equal_ast(a->data.AstAnd.r1, b->data.AstAnd.r1) &&
           equal_ast(a->data.AstAnd.r2, b->data.AstAnd.r2);
  case OrNode:
    return equal_ast(a->data.AstOr.r1, b->data.AstOr.r1) &&
           equal_ast(a->data.AstOr.r2, b->data.AstOr.r2);
  case RepeatNode:
    return equal_ast(a->data.AstRepeat.r, b->data.AstRepeat.r);
  case SurroundNode:
    return equal_ast(a->data.AstSurround.r, b->data.AstSurround.r);
  default:
    return false;
  }
}

/* free */
void free_ast(Ast *node) {
  if (node == NULL)
    return;
  switch (node->type) {
  case LiteralNode:
    /* nothing to free */
    break;
  case SetNode:
    /* nothing to free */
    break;
  case AndNode:
    free_ast(node->data.AstAnd.r1);
    free_ast(node->data.AstAnd.r2);
    break;
  case OrNode:
    free_ast(node->data.AstOr.r1);
    free_ast(node->data.AstOr.r2);
    break;
  case RepeatNode:
    free_ast(node->data.AstRepeat.r);
    break;
  case SurroundNode:
    free_ast(node->data.AstSurround.r);
    break;
  }
  free(node);
}

/*
 * ============================================================================
 * builder/parser.c - Parser for regular expressions
 * ============================================================================
 */


/* pre-define */
Vector_char *new_vector_char();
int push_vector_char(Vector_char *vec, char value);

typedef struct Parser {
  Lexer *lexer;
  Token *current_token;
} Parser;

Parser *new_parser(Lexer *lexer) {
  Parser *parser = (Parser *)malloc(sizeof(Parser));
  parser->lexer = lexer;
  parser->current_token = get_next_token(lexer);
  return parser;
}

static void eat(Parser *parser, TokenType type) {
  if (parser->current_token->type == type) {
    parser->current_token = get_next_token(parser->lexer);
  } else {
    printf("Wrong Token Type! Expect %d, found %d.\n", type,
           parser->current_token->type);
    exit(1);
  }
}

static char eat_escape_char(Parser *parser) {
  eat(parser, BACK_SLASH);
  char value;
  switch (parser->current_token->value) {
  case 'a': /* beep */
    value = '\a';
    break;
  case 'n': /* newline */
    value = '\n';
    break;
  case 'r': /* carriage return */
    value = '\r';
    break;
  case 't': /* tab */
    value = '\t';
    break;
  default:
    value = parser->current_token->value;
  }
  parser->current_token = get_next_token(parser->lexer);
  return value;
}

/* Forward declarations */
static Ast *parse_expr(Parser *parser);
static Ast *parse_term(Parser *parser);
static Ast *parse_factor(Parser *parser);
static Ast *parse_base(Parser *parser);
static Ast *parse_range(Parser *parser);

/*
 * expr := term ('|' term)*
 */
static Ast *parse_expr(Parser *parser) {
  Ast *node = parse_term(parser);
  while (parser->current_token->type == BAR) {
    eat(parser, BAR);
    Ast *right = parse_term(parser);
    node = new_ast_or(node, right);
  }
  return node;
}

/*
 * term := factor*
 */
static Ast *parse_term(Parser *parser) {
  Ast *node = parse_factor(parser);
  while (parser->current_token->type == LITERAL ||
         parser->current_token->type == CARET ||
         parser->current_token->type == DOT ||
         parser->current_token->type == LBRACKET ||
         parser->current_token->type == LPAREN ||
         parser->current_token->type == BACK_SLASH) {
    Ast *right = parse_factor(parser);
    node = new_ast_and(node, right);
  }
  return node;
}

/*
 * factor := base '*'
 *           base '+'
 */
static Ast *parse_factor(Parser *parser) {
  Ast *node = parse_base(parser);
  if (parser->current_token->type == ASTERISK) {
    eat(parser, ASTERISK);
    node = new_ast_repeat(node);
  } else if (parser->current_token->type == PLUS) {
    eat(parser, PLUS);
    node = new_ast_and(clone_ast(node), new_ast_repeat(node));
  }
  return node;
}

/*
 * base := LITERAL | CARET
 *       | BACK_SLASH any_single_character
 *       | DOT
 *       | '[' range ']'
 *       | '(' expr ')'
 */
static Ast *parse_base(Parser *parser) {
  switch (parser->current_token->type) {
  case LITERAL: {
    char value = parser->current_token->value;
    eat(parser, LITERAL);
    return new_ast_literal(value);
  }
  case CARET: {
    char value = parser->current_token->value;
    eat(parser, CARET);
    return new_ast_literal(value);
  }
  case BACK_SLASH: {
    return new_ast_literal(eat_escape_char(parser));
  }
  case DOT: {
    /* anything but newline */
    eat(parser, DOT);
    Vector_char *set = new_vector_char();
    push_vector_char(set, '\n');
    return new_ast_set(set, true);
  }
  case LBRACKET: {
    eat(parser, LBRACKET);
    Ast *node = parse_range(parser);
    eat(parser, RBRACKET);
    return node;
  }
  case LPAREN: {
    eat(parser, LPAREN);
    Ast *node = parse_expr(parser);
    eat(parser, RPAREN);
    return new_ast_surround(node);
  }
  default: {
    printf("unexpected token: %d\n", parser->current_token->type);
    exit(1);
  }
  }
}

/*
 * range or set
 * range := CARET
 *        | LITERAL
 *        | BACK_SLASH any_single_character
 *        | LITERAL DASH LITERAL
 *        | range
 */
static Ast *parse_range(Parser *parser) {
  /* parse negate ^ */
  bool is_neg = false;
  if (parser->current_token->type == CARET) {
    is_neg = true;
    eat(parser, CARET);
  }

  Vector_char *set = new_vector_char();
  while (parser->current_token->type != RBRACKET) {
    if (parser->current_token->type == BACK_SLASH) {
      push_vector_char(set, eat_escape_char(parser));
    } else {
      char from = parser->current_token->value;
      eat(parser, LITERAL);
      Ast *right;
      /* range */
      if (parser->current_token->type == DASH) {
        eat(parser, DASH);
        char to = parser->current_token->value;
        eat(parser, LITERAL);
        for (char c = from; c <= to; ++c)
          push_vector_char(set, c);
      }
      /* set */
      else
        push_vector_char(set, from);
    }
  }
  return new_ast_set(set, is_neg);
}

/* Entry point for parsing */
Ast *parse(Parser *parser) {
  Ast *node = parse_expr(parser);
  if (parser->current_token->type != END) {
    printf("unexpected trailing token: %d\n", parser->current_token->type);
    exit(1);
  }
  return node;
}

/*
 * ============================================================================
 * edge.c - Edges between states
 * ============================================================================
 */

#define TYPE char
/*
 * create new, push, free functions for TYPE
 * usage:
 *   #define TYPE int
 *   #include <this_file>
 */

#ifndef TYPE
#error "define TYPE first"
#else

#define CAT1(a, b) a##b
#define CAT(a, b) CAT1(a, b)
#define APPEND_TYPE(a) CAT(CAT(a, _), TYPE)
#define TYPE_NAME APPEND_TYPE(Vector)

#include <stddef.h>
#include <stdlib.h>

typedef struct TYPE_NAME {
  TYPE *data;
  size_t size;
  size_t capacity;
} TYPE_NAME;

/* create a new vector */
TYPE_NAME *APPEND_TYPE(new_vector)() {
  TYPE_NAME *vec = (TYPE_NAME *)malloc(sizeof(TYPE_NAME));
  if (!vec)
    return NULL;
  vec->size = 0;
  vec->capacity = 4;
  vec->data = (TYPE *)malloc(vec->capacity * sizeof(TYPE));
  if (!vec->data) {
    free(vec);
    return NULL;
  }
  return vec;
}

/* push a value to the vector */
int APPEND_TYPE(push_vector)(TYPE_NAME *vec, TYPE value) {
  if (vec->size >= vec->capacity) {
    size_t new_capacity = vec->capacity * 2;
    TYPE *new_data = (TYPE *)realloc(vec->data, new_capacity * sizeof(TYPE));
    if (!new_data)
      return -1;
    vec->data = new_data;
    vec->capacity = new_capacity;
  }
  vec->data[vec->size++] = value;
  return 0;
}

/* free the vector */
void APPEND_TYPE(free_vector)(TYPE_NAME *vec) {
  if (vec) {
    free(vec->data);
    free(vec);
  }
}

#undef TYPE
#endif /* ifndef TYPE */

typedef struct Label {
  enum {
    CHAR,
    SET,
    NEG_SET,
  } type;

  union {
    char symbol;
    Vector_char *set;
  } data;
} Label;

Label *new_literal_label(char symbol) {
  Label *label = (Label *)malloc(sizeof(Label));
  label->type = CHAR;
  label->data.symbol = symbol;
  return label;
}

Label *new_set_label(Vector_char *set, bool is_neg) {
  Label *label = (Label *)malloc(sizeof(Label));
  label->type = (is_neg ? NEG_SET : SET);
  label->data.set = set;
  return label;
}

typedef struct Edge {
  Label *label;
  State from;
  State to;
} Edge;

/* create a new edge */
Edge *new_edge(Label *label, State from, State to) {
  Edge *e = (Edge *)malloc(sizeof(Edge));
  e->label = label;
  e->from = from;
  e->to = to;
  return e;
}

/*
 * ============================================================================
 * nfa.c - NFA (Non-deterministic Finite Automaton) implementation
 * ============================================================================
 */

char EPSILON = -1;

typedef struct NFA {
  State states_count;
  States *target_states;
  Edge *edges[MAX];
  unsigned int edges_count;
} NFA;

/* create a new NFA */
NFA *new_nfa() {
  NFA *nfa = (NFA *)malloc(sizeof(NFA));
  nfa->states_count = 0;
  nfa->target_states = NULL;
  nfa->edges_count = 0;
  return nfa;
}

/* set the states count of an NFA */
void set_states_count(NFA *nfa, State states_count) {
  nfa->states_count = states_count;
}

/* set the target states of an NFA */
void set_target_states(NFA *nfa, States *s) { nfa->target_states = s; }

/* add an edge to an NFA */
void push_edge(NFA *nfa, Edge *e) {
  nfa->edges[nfa->edges_count] = e;
  ++(nfa->edges_count);
}

/* print edges in the form of `from --symbol--> to` */
void print_edges(NFA *nfa) {
  printf("=== NFA\n");
  for (size_t i = 0; i < nfa->edges_count; ++i) {
    Edge *e = nfa->edges[i];
    Label *l = e->label;
    if (l->type == CHAR) {
      char symbol = l->data.symbol;
      if (symbol == EPSILON)
        printf("%2d ---ε---> %2d\n", e->from, e->to);
      else
        printf("%2d ---%c---> %2d\n", e->from, symbol, e->to);
    } else if (l->type == SET || NEG_SET) {
      printf("%2d --", e->from);
      if (l->type == NEG_SET)
        printf("^");
      for (size_t i = 0; i < e->label->data.set->size; ++i) {
        char c = e->label->data.set->data[i];
        if (c == '\n')
          printf("↵");
        else
          printf("%c", c);
      }
      printf("--> %2d\n", e->to);
    }
  }
}

/* free an NFA */
void free_nfa(NFA *nfa) {
  /* free edges */
  for (size_t i = 0; i < nfa->edges_count; ++i) {
    free(nfa->edges[i]->label);
    free(nfa->edges[i]);
  }
  /* free target states */
  if (nfa->target_states != NULL) {
    free(nfa->target_states);
  }
  /* free nfa */
  free(nfa);
  nfa = NULL;
}

static bool accept(Label *label, char input) {
  if (input == EPSILON)
    return label->type == CHAR && label->data.symbol == EPSILON;

  switch (label->type) {
  case CHAR:
    return input == label->data.symbol;
  case SET:
    for (size_t i = 0; i < label->data.set->size; ++i)
      if (label->data.set->data[i] == input)
        return true;
    return false;
  case NEG_SET:
    for (size_t i = 0; i < label->data.set->size; ++i)
      if (label->data.set->data[i] == input)
        return false;
    return true;
  }
  exit(EXIT_FAILURE);
}

/* return all states reachable with epsilon labels from the given states */
States *epsilon_closure(NFA *nfa, States *s) {
  States *new_s = new_states();

  /* add all original states to the closure first */
  for (size_t i = 0; i < s->len; ++i) {
    push_state(new_s, s->states[i]);
  }

  /* simulate the original states as a stack */
  while (s->len > 0) {
    --(s->len);
    State state = s->states[s->len];
    for (size_t i = 0; i < nfa->edges_count; ++i) {
      Edge *e = nfa->edges[i];
      if (e->from == state && accept(e->label, EPSILON)) {
        State next_state = e->to;
        if (!have_state(new_s, next_state)) {
          push_state(new_s, next_state);
          s->states[s->len] = next_state;
          ++(s->len);
        }
      }
    }
  }

  free(s);
  return new_s;
}

/* return all states reachable with given symbol from the given states */
States *move(NFA *nfa, States *s, char symbol) {
  States *new_s = new_states();
  for (size_t i = 0; i < s->len; ++i) {
    for (size_t j = 0; j < nfa->edges_count; ++j) {
      Edge *e = nfa->edges[j];
      if (e->from == s->states[i] && accept(e->label, symbol)) {
        State next_state = e->to;
        if (!have_state(new_s, next_state)) {
          push_state(new_s, next_state);
        }
      }
    }
  }
  free(s);
  return new_s;
}

/*
 * ============================================================================
 * builder.c - Build an NFA from a string
 * ============================================================================
 */

static State g_state_counts = 0;

typedef struct {
  NFA *nfa;
  State start;
  State accept;
} NFAFragment;

/* create a new NFA fragment */
static NFAFragment *new_nfa_fragment(NFA *nfa, State start, State accept) {
  NFAFragment *fragment = (NFAFragment *)malloc(sizeof(NFAFragment));
  fragment->nfa = nfa;
  fragment->start = start;
  fragment->accept = accept;
  return fragment;
}

/* get current state counts */
static State get_state_counts() { return g_state_counts; }

/* increase states counts and get the latest state number */
static State increase_state_counts() { return g_state_counts++; }

/* decrease states counts by one, used to concatenate two NFA */
static void decrease_state_counts() { --g_state_counts; }

/* add an ε-labled edge to the NFA */
static void add_epsilon(NFA *nfa, State from, State to) {
  push_edge(nfa, new_edge(new_literal_label(EPSILON), from, to));
}

/* add a symbol-labled edge to the NFA */
static void add_symbol(NFA *nfa, State from, State to, char symbol) {
  push_edge(nfa, new_edge(new_literal_label(symbol), from, to));
}

static void add_set(NFA *nfa, State from, State to, Vector_char *set,
                    bool is_neg) {
  push_edge(nfa, new_edge(new_set_label(set, is_neg), from, to));
}

/* move all edges from source NFA to destination NFA */
static void move_edges(NFA *dst, NFA *src) {
  for (size_t i = 0; i < src->edges_count; ++i) {
    push_edge(dst, src->edges[i]);
  }
  src->edges_count = 0;
}

static NFAFragment *ast2nfa_fragment(Ast *ast) {
  if (ast == NULL)
    return NULL;

  switch (ast->type) {
  case LiteralNode: {
    /* START --literal--> END */
    NFA *nfa = new_nfa();
    State start = increase_state_counts();
    State accept = increase_state_counts();
    add_symbol(nfa, start, accept, ast->data.AstLiteral.value);
    return new_nfa_fragment(nfa, start, accept);
  }

  case SetNode: {
    /* START --set--> END */
    NFA *nfa = new_nfa();
    State start = increase_state_counts();
    State accept = increase_state_counts();
    add_set(nfa, start, accept, ast->data.AstSet.set, ast->data.AstSet.is_neg);
    return new_nfa_fragment(nfa, start, accept);
  }

  case AndNode: {
    /* START --left--> (left end & right start) --right--> END */
    NFAFragment *left = ast2nfa_fragment(ast->data.AstAnd.r1);
    decrease_state_counts(); /* concatenate left end and right start */
    NFAFragment *right = ast2nfa_fragment(ast->data.AstAnd.r2);
    move_edges(left->nfa, right->nfa);
    free(right->nfa);
    NFAFragment *result =
        new_nfa_fragment(left->nfa, left->start, right->accept);
    free(left);
    free(right);
    return result;
  }

  case OrNode: {
    /*
     *          /-ε--> S₀ --left---> S₁ -ε-\
     * START --<                            >--> END
     *          \-ε--> S₂ --right--> S₃ -ε-/
     */
    NFA *nfa = new_nfa();
    State start = increase_state_counts();
    NFAFragment *left = ast2nfa_fragment(ast->data.AstOr.r1);
    NFAFragment *right = ast2nfa_fragment(ast->data.AstOr.r2);
    State accept = increase_state_counts();
    add_epsilon(nfa, start, left->start);
    add_epsilon(nfa, start, right->start);
    add_epsilon(nfa, left->accept, accept);
    add_epsilon(nfa, right->accept, accept);
    move_edges(nfa, left->nfa);
    move_edges(nfa, right->nfa);
    free(left->nfa);
    free(right->nfa);
    NFAFragment *result = new_nfa_fragment(nfa, start, accept);
    free(left);
    free(right);
    return result;
  }

  case RepeatNode: {
    /*
     *               .-<-ε-<-.
     *              /         \
     * START --ε--> S₀ --r--> S₁ --ε--> END
     *     \                            /
     *      .---------->-ε->-----------.
     */
    NFA *nfa = new_nfa();
    State start = increase_state_counts();
    NFAFragment *body = ast2nfa_fragment(ast->data.AstRepeat.r);
    State accept = increase_state_counts();
    move_edges(nfa, body->nfa);
    add_epsilon(nfa, start, body->start);
    add_epsilon(nfa, start, accept);
    add_epsilon(nfa, body->accept, body->start);
    add_epsilon(nfa, body->accept, accept);
    free(body->nfa);
    NFAFragment *result = new_nfa_fragment(nfa, start, accept);
    free(body);
    return result;
  }

  case SurroundNode: {
    /* START --r--> END */
    return ast2nfa_fragment(ast->data.AstSurround.r);
  }

  default:
    exit(1);
  }
}

NFA *ast2nfa(Ast *ast) {
  NFAFragment *fragment = ast2nfa_fragment(ast);
  NFA *nfa = fragment->nfa;
  free(fragment);
  free_ast(ast);

  nfa->states_count = g_state_counts;

  States *target_states = new_states();
  push_state(target_states, g_state_counts - 1);
  nfa->target_states = target_states;
  return nfa;
}

NFA *build(char *pattern) {
  Lexer *lexer = new_lexer(pattern);
  Parser *parser = new_parser(lexer);
  Ast *ast = parse(parser);
  NFA *nfa = ast2nfa(ast);
  if (lexer->current_token != NULL) {
    free(lexer->current_token);
  }
  free(lexer);
  free(parser);
  return nfa;
}

NFA *build_many(char **patterns, size_t len) {
  g_state_counts = 0;
  NFA *nfa = new_nfa();
  nfa->target_states = new_states();
  State start = increase_state_counts();

  for (size_t i = 0; i < len; ++i) {
    State sub_start = get_state_counts();
    NFA *sub_nfa = build(patterns[i]);
    move_edges(nfa, sub_nfa);
    add_epsilon(nfa, start, sub_start);
    push_state(nfa->target_states, sub_nfa->target_states->states[0]);
    free(sub_nfa);
  }

  return nfa;
}

/*
 * ============================================================================
 * match.c - Functions to match string with patterns
 * ============================================================================
 */

typedef unsigned long IdxType;

/* if the input string fully matches the pattern */
bool match_full(NFA *nfa, char *input) {
  States *s = new_states();
  push_state(s, 0);
  s = epsilon_closure(nfa, s);

  char *next_char = input;
  while (*next_char != '\0') {
    s = epsilon_closure(nfa, move(nfa, s, *next_char));
    ++next_char;
  }
  bool result = get_shared_states(s, nfa->target_states);
  free(s);
  return result;
}

/*
 * find the first longest match, and copy it to (char *)text, return its length
 */
IdxType match(NFA *nfa, char *input, char *text) {
  States *s = new_states();
  push_state(s, 0);
  s = epsilon_closure(nfa, s);

  IdxType len = 0;
  IdxType last_match = 0;
  char *next_char = input;
  while (*next_char != '\0') {
    s = epsilon_closure(nfa, move(nfa, s, *next_char));

    /*
     * if nothing matches, return to the starting state.
     * however, if there is any match, stop matching and return it as the
     * longest match
     */
    if (states_is_empty(s)) {
      if (last_match > 0)
        break;
      else {
        push_state(s, 0);
        s = epsilon_closure(nfa, s);
      }
    } else {
      text[(len)++] = *next_char;
    }

    /* if any target state is reached, mark matching */
    if (get_shared_states(s, nfa->target_states))
      last_match = len;

    ++next_char;
  }
  len = last_match;
  text[len] = '\0';
  free(s);
  return len;
}

/*
 * similar to `match`, but copy to yytext, assign its length to yyleng,
 * and return the index of the pattern matched
 */
int yy_match(NFA *nfa) {
  States *s = new_states();
  push_state(s, 0);
  s = epsilon_closure(nfa, s);

  yyleng = 0;
  IdxType last_match = 0;
  State first_shared_state = 0;
  while (g_buffer_ptr < g_buffer + g_buflen) {
    s = epsilon_closure(nfa, move(nfa, s, *g_buffer_ptr));

    if (states_is_empty(s)) {
      if (last_match > 0)
        break;
      else {
        push_state(s, 0);
        s = epsilon_closure(nfa, s);
      }
    } else {
      yytext[(yyleng)++] = *g_buffer_ptr;
    }

    /* if any target state is reached, mark matching */
    if ((first_shared_state = get_shared_states(s, nfa->target_states))) {
      last_match = yyleng;
    }

    ++g_buffer_ptr;
  }
  yyleng = last_match;
  yytext[yyleng] = '\0';

  for (size_t i = 0; i < nfa->target_states->len; ++i) {
    if (nfa->target_states->states[i] == first_shared_state) {
      free(s);
      return i;
    }
  }

  /* unreachable */
  exit(EXIT_FAILURE);
}

"#;

pub const YYLEX: &str = r#"
int yylex() {
  if (yyin == NULL)
    yyin = stdin;
  if (yyout == NULL)
    yyout = stdout;

  yy_read_buffer();
  g_buffer_ptr = g_buffer;
  NFA *nfa = build_many(g_patterns, g_pattern_count);
  while (g_buffer_ptr < g_buffer + g_buflen) {
    int pattern_idx = yy_match(nfa);
    action(pattern_idx);
  }
  return 0;
}"#;
