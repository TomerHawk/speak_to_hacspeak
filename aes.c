#include "aes.h"
#include <stdio.h>


int main(int argc, char const *argv[]) {
  unsigned char* message = malloc(sizeof(unsigned char)*32);
  unsigned char* key = malloc(sizeof(unsigned char)*32);

  printf("Give me a message: ");
  scanf("%s",message );
  printf("Give me a key: ");
  scanf("%s",key );

  teso_aes256_1(message,key);
  return 0;
}
