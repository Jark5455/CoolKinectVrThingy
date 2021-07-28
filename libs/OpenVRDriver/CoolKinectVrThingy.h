#include <cstdint>

extern "C" {

void deactivate();

void CoolKinectVrThingy();

uint32_t getJoystickX();

uint32_t getJoystickY();

bool getJoystickC();

bool getJoystickZ();

float getHeadX();

float getHeadY();

float getHeadZ();

float getLeftHandX();

float getLeftHandY();

float getLeftHandZ();

float getRightHandX();

float getRightHandY();

float getRightHandZ();

float getLeftElbowX();

float getLeftElbowY();

float getLeftElbowZ();

float getRightElbowX();

float getRightElbowY();

float getRightElbowZ();

} // extern "C"
