#include "ControllerDriver.h"

EVRInitError ControllerDriver::Activate(uint32_t unObjectId){
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
  pose.qWorldFromDriverRotation = quat;
  
  return pose;
}

void ControllerDriver::RunFrame(){
  
}