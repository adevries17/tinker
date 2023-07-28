#include <stdio.h>
#include <string.h>

int main() {
  char pw[] = "doubleK";
  char passwd[32];

  printf("Enter the password: ");
  scanf(" %s", passwd);

  int result = strcmp(pw, passwd);

  if (result == 0) {
    printf("Correct!");
  } else {
    printf("WRONG!");
  }

  return 0;
}
