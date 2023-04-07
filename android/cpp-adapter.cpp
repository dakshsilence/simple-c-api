#include <jni.h>
#include <simple.h>
#include <stdlib.h>
#include <string.h>

#define M_(name) Java_com_simple_Native_##name
// extern "C" JNIEXPORT jint JNICALL
// Java_com_simple_SimpleModule_nativeMultiply(JNIEnv *env, jclass type, jdouble a, jdouble b)
// {
//     return a * b;
//     // return simple::multiply(a, b);
// }

extern "C" JNIEXPORT jint JNICALL
M_(getRandom)(JNIEnv *env, jclass type)
{
    return get_random();
}

extern "C" JNIEXPORT jbyteArray JNICALL
M_(getRandomNumber)(JNIEnv *env, jclass type, jint n)
{
    Buffer buf = get_random_number(n);
    jbyteArray _bytes = env->NewByteArray(buf.size);
    env->SetByteArrayRegion(_bytes, 0, buf.size, (jbyte *)buf.ptr);
    free_buffer(buf);
    return _bytes;
}

extern "C" JNIEXPORT jstring JNICALL
M_(getRandomString)(JNIEnv *env, jclass type, jint n)
{
    Buffer buf = get_random_number(n);
    jstring m_value;
    const unsigned char *data = buf.ptr;
    int size = buf.size;
    char *newData = (char *)malloc(size + 1);
    for (int i = 0; i < size; i++)
    {
        newData[i] = data[i] - 127;
    }
    // env->ReleaseStringChars(m_value, newData);
    return (env)->NewStringUTF((char *)newData);
}