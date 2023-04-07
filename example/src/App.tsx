import * as React from 'react';

import { StyleSheet, View, Button } from 'react-native';
import {
  getRandom,
  getRandomBytes,
  getRandomNumber,
} from 'react-native-simple';

export default function App() {
  return (
    <View style={styles.container}>
      <Button
        title={'test'}
        onPress={async () => {
          console.log('Button Pressed', await getRandomBytes(3));
        }}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
  },
  box: {
    width: 60,
    height: 60,
    marginVertical: 20,
  },
});
