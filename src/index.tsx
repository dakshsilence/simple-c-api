import { NativeModules, Platform } from 'react-native';

const LINKING_ERROR =
  `The package 'react-native-simple' doesn't seem to be linked. Make sure: \n\n` +
  Platform.select({ ios: "- You have run 'pod install'\n", default: '' }) +
  '- You rebuilt the app after installing the package\n' +
  '- You are not using Expo Go\n';

const Simple = NativeModules.Simple
  ? NativeModules.Simple
  : new Proxy(
      {},
      {
        get() {
          throw new Error(LINKING_ERROR);
        },
      }
    );

export async function getRandom(): Promise<number> {
  return await Simple.getRandom();
}

export async function getRandomNumber(n: number): Promise<string> {
  return await Simple.getRandomNumber(n);
}

export async function getRandomBytes(n: number): Promise<string> {
  return await Simple.getRandomBytes(n);
}
