include CommonDefs.mak

BIN_DIR = ../../Bin

INC_DIRS = \
	../Common \
	../../Include \
	/opt/kinect/OpenNI-2.2-α/Linux-x64/OpenNI-Linux-x64-2.2.0/Include/

SRC_FILES = *.cpp

LIB_DIRS += $(OPENNI2_REDIST)

USED_LIBS += OpenNI2 NiTE2

EXE_NAME = SimpleUserTracker

CFLAGS += -fPIC -Wall

include CommonCppMakefile
