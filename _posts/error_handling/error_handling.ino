#include "error_handling.h"

const int LED_PIN = 13;
uint8_t   brightness = 0;
int fadeAmount = 5;

void setup() {
  pinMode(LED_PIN, OUTPUT);
  Serial.begin(9600);

  /* Hypothetical function where things can go wrong */
  init_magic();
}

void init_magic() {
  long randNumber;
  // nothing connected to 0 so read sees noise
  randomSeed(analogRead(0));
  randNumber = random(8);

  switch (randNumber) {
    case 0:
      error_blink(MEM_LOAD_FAILED, "Could not load from memory!");
      break;
    case 1:
      error_blink(CONNECTION_REFUSED, "Could not establish connection!");
      break;
    case 2:
      error_blink(DEADBEEF, "0xDEADBEEF ERROR!");
      break;
    default:
      {
        /* Construct an error message with an integer error code in it */
        char msg[60] = "Unknown error occurred. Code:    ";
        itoa(randNumber, msg + 30, 10);
        error_blink(ERROR_UNKNOWN, msg);
        break;
      }
  }
}

/* Just the fade example from arduino to have the sketch do something.
   What matters is in setup only!
*/
void loop() {
  analogWrite(LED_PIN, brightness);
  brightness = brightness + fadeAmount;

  // reverse the direction of the fading at the ends of the fade:
  if (brightness <= 0 || brightness >= 255) {
    fadeAmount = -fadeAmount;
  }
  delay(30);
}
