#include <jni.h>
#include "react-native-simple.h"

extern "C"
JNIEXPORT jint JNICALL
Java_com_simple_SimpleModule_nativeMultiply(JNIEnv *env, jclass type, jdouble a, jdouble b) {
    return simple::multiply(a, b);
}
