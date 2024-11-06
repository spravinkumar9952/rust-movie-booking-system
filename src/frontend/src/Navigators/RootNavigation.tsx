import React from "react";
import OnboardingFlow from "./OnboardingFlow";
import { HomeNavigation } from "./HomeNavigation";
import {NavigationContainer} from '@react-navigation/native';
import { createStackNavigator } from "@react-navigation/stack";
import { useAuth } from "../Context/AuthContext";


const RootStack = createStackNavigator();

export const RootNavigation: React.FC = () => {
  const { registrationToken } = useAuth();

  return (
    <NavigationContainer>
      <RootStack.Navigator screenOptions={{ headerShown: false }}>
        {registrationToken ? (
          <RootStack.Screen name="HomeNavigation" component={HomeNavigation} />
        ) : (
          <RootStack.Screen name="OnboardingFlow" component={OnboardingFlow} />
        )}
      </RootStack.Navigator>
    </NavigationContainer>
  );
};