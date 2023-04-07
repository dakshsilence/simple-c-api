package com.simple;

import androidx.annotation.NonNull;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.module.annotations.ReactModule;

@ReactModule(name = SimpleModule.NAME)
public class SimpleModule extends ReactContextBaseJavaModule {
  public static final String NAME = "Simple";

  public SimpleModule(ReactApplicationContext reactContext) {
    super(reactContext);
  }

  @Override
  @NonNull
  public String getName() {
    return NAME;
  }

  @ReactMethod
  public void getRandom(Promise promise) {
    promise.resolve(Native.Simple.getRandom());
  }

  @ReactMethod
  public void getRandomNumber(int n,Promise promise) {
    promise.resolve(Native.Simple.getRandomString(n));
  }

  @ReactMethod
  public void getRandomBytes(int n,Promise promise) {
    // Log.d("Native.Simple.getRandomNumber(n)");
    promise.resolve(Native.Simple.getRandomNumber(n));
  }

}
