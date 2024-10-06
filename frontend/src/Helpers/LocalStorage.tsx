
import AsyncStorage from '@react-native-async-storage/async-storage';

export const storeKeyVal = async (key: string, val: string) => {
  try{
    await AsyncStorage.setItem(key, val);
  }catch(e){
    console.log(e);
  }
}

export const getVal = async (key: string) => {
  try{
    await AsyncStorage.getItem(key);
  }catch(e){
    console.log(e);
  }
}