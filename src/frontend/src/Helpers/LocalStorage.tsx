
import AsyncStorage from '@react-native-async-storage/async-storage';

export const storeKeyVal = async (key: string, val: string) => {
  try{
    await AsyncStorage.setItem(key, val);
  }catch(e){
    console.log(e);
  }
}

export const getVal = async (key: string): Promise<string | null>=> {
  try{
    return await AsyncStorage.getItem(key);
  }catch(e){
    console.log(e);
  }
  return null
}

export const removeKey = async (key: string) => {
  try{
    await AsyncStorage.removeItem(key);
  }catch(e){
    console.log(e);
  }
}