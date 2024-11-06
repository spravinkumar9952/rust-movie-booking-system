import { Pressable, Text, View } from "react-native";
import { removeKey } from "../Helpers/LocalStorage";
import { BottomTabNavigationProp } from "@react-navigation/bottom-tabs";
import { FC } from "react";

type ProfileProps = {
  navigation: BottomTabNavigationProp<any>;
}

export const Profile : FC<ProfileProps>= ({navigation}) => {
  const handleLogoutPress = () => {
    console.log("Logout Pressed");
    removeKey("registration_token");
    navigation.reset({
      index: 0,
      routes: [{ name: 'OnboardingFlow' }],
    });
  }

  return (
    <View>
      <Pressable onPress={handleLogoutPress}>
        <Text>Logout</Text>
      </Pressable>
    </View>
  );
}


