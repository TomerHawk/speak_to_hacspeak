#include "aes.h"
#include <stdio.h>


int main(int argc, char const *argv[]) {
  unsigned char* message = malloc(sizeof(unsigned char)*32);
  unsigned char* key = malloc(sizeof(unsigned char)*32);

  printf("Give me a message: ");
  scanf("%s",message );
  printf("Give me a key: ");
  scanf("%s",key );

  uint16_t* array = teso_aes256_1(message,key);
  int i = 0;
  while(i < 32){
    printf("%hu\n",array[i]);
    i++;
  }
  return 0;
}
