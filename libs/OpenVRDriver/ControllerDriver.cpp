#include "ControllerDriver.h"
#include "CoolKinectVrThingy.h"

#include <string.h>

EVRInitError ControllerDriver::Activate(uint32_t unObjectId){
  
  CoolKinectVrThingy();
  
  driverId = unObjectId;
  PropertyContainerHandle_t props = VRProperties()->TrackedDeviceToPropertyContainer(driverId);
  
  VRProperties()->SetStringProperty(props, Prop_InputProfilePath_String, "nite4vr/input/controller_profile.json");
  VRProperties()->SetInt32Property(props, vr::Prop_ControllerRoleHint_Int32, ETrackedControllerRole::TrackedControllerRole_RightHand);
  VRDriverInput()->CreateScalarComponent(props, "/input/joystick/y", &joystickYHandle, EVRScalarType::VRScalarType_Absolute, EVRScalarUnits::VRScalarUnits_NormalizedTwoSided);
  VRDriverInput()->CreateScalarComponent(props, "/input/joystick/x", &joystickXHandle, EVRScalarType::VRScalarType_Absolute, EVRScalarUnits::VRScalarUnits_NormalizedTwoSided);
    
  VRDriverInput()->CreateBooleanComponent(props, "/input/buttons/c", &buttonC);
  VRDriverInput()->CreateBooleanComponent(props, "/input/buttons/z", &buttonZ);
  
  return VRInitError_None;
}

DriverPose_t ControllerDriver::GetPose() {
  DriverPose_t pose = { 0 };
  pose.poseIsValid = false;
  pose.result = vr::TrackingResult_Calibrating_OutOfRange;
  pose.deviceIsConnected = true;
  
  HmdQuaternion_t quat;
  quat.w = 1;
  quat.x = 0;
  quat.y = 0;
  quat.z = 0;
  
  pose.qWorldFromDriverRotation = quat;
  pose.qDriverFromHeadRotation = quat;
  
  return pose;
}

void ControllerDriver::RunFrame(){
  VRDriverInput()->UpdateScalarComponent(joystickXHandle, (getJoystickX() - 30.0) / 100.0, 0);
  VRDriverInput()->UpdateScalarComponent(joystickYHandle, (getJoystickY() - 30.0) / 100.0, 0);
  
  VRDriverInput()->UpdateBooleanComponent(buttonC, getJoystickC(), 0);
  VRDriverInput()->UpdateBooleanComponent(buttonZ, getJoystickZ(), 0);
}

void ControllerDriver::Deactivate() {
  driverId = k_unTrackedDeviceIndexInvalid;
  deactivate();
}

void* ControllerDriver::GetComponent(const char* pchComponentNameAndVersion)
{
	if (strcmp(IVRDriverInput_Version, pchComponentNameAndVersion) == 0)
	{
		return this;
	}
	return NULL;
}

void ControllerDriver::EnterStandby() {}

void ControllerDriver::DebugRequest(const char* pchRequest, char* pchResponseBuffer, uint32_t unResponseBufferSize) 
{
	if (unResponseBufferSize >= 1)
	{
		pchResponseBuffer[0] = 0;
	}
}