pub const PREPARE: &str = r#"
#include <stdio.h>
#include <stdlib.h>

typedef unsigned long IdxType;
char *g_buffer;
IdxType g_buflen;
IdxType g_bufidx;

char* yytext = "YYTEXT"; /* TODO: implement yytext properly */

void read_file(const char *filename) {
  FILE *fp = fopen(filename, "r");
  if (fp == NULL) {
    perror("Failed to open file");
  }

  fseek(fp, 0, SEEK_END);
  g_buflen = ftell(fp);
  rewind(fp);

  g_buffer = malloc(g_buflen + 1);
  if (!g_buffer) {
    perror("Failed to allocate memory for buffer");
  }

  fread(g_buffer, 1, g_buflen, fp);
  g_buffer[g_buflen] = '\0'; // Null-terminate the string
  fclose(fp);
}

"#;

pub const MATCH: &str = r#"
void match() {
  while (g_bufidx < g_buflen) {
    for (int i = 0; i < g_pattern_count; i++) {
      const char *pat = g_patterns[i];
      if (g_buffer[g_bufidx] == pat[0]) {
        IdxType current_idx = g_bufidx;
        while (g_buffer[current_idx] == pat[current_idx - g_bufidx] &&
               pat[current_idx - g_bufidx] != '\0') {
          current_idx++;
        }
        if (pat[current_idx - g_bufidx] == '\0') {
          action(i);
        }
      }
    }
    ++g_bufidx;
  }
}

"#;
