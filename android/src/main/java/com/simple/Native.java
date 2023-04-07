package com.simple;

final class Native {
  static final Native Simple;

  static {
    Simple = new Native();
    System.loadLibrary("rnsimple");
  }

  public native int getRandom();
  public native String getRandomString(int n);
  public native byte[] getRandomNumber(int n);
}