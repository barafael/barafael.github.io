static const uint16_t BLINK_PERIOD = 500;

typedef enum {
  MEM_LOAD_FAILED,
  CONNECTION_REFUSED,
  DEADBEEF,
  ERROR_UNKNOWN
} error_type;

void blink_pattern(char pattern[]) {
  while (1) {
    for (size_t index = 0; index < strlen(pattern); index++) {
      switch (pattern[index]) {
        case '0':
          digitalWrite(13, LOW);
          delay(BLINK_PERIOD);
          break;
        case '1':
          digitalWrite(13, HIGH);
          delay(BLINK_PERIOD);
          break;
        default:
          Serial.println(F("Invalid pattern string!"));
          return;
      }
    }
  }
}

void error_blink(error_type error, char message[]) {
  Serial.println(message);
  switch (error) {
    case MEM_LOAD_FAILED:
      blink_pattern("1100");
      break;
    case CONNECTION_REFUSED:
      blink_pattern("1010");
      break;
    case DEADBEEF:
      blink_pattern("0001");
      break;
    case ERROR_UNKNOWN:
      blink_pattern("00010101");
      break;
    default:
      Serial.print(F("Error not tagged! Code: "));
      Serial.println(error);
      blink_pattern("10100000");
      break;
  }
}
