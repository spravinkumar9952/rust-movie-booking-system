



import * as React from 'react';
import { createStackNavigator } from '@react-navigation/stack';
import Register from '../Screens/Register';
import Login from '../Screens/Login';
import { Movies } from '../Screens/Movies';
import { HomeNavigation } from './HomeNavigation';

const Stack = createStackNavigator();

export const OnboardingFlow = () => {
  return (
      <Stack.Navigator>
        <Stack.Screen
          name="Login"
          component={Login}
          options={{ headerShown : false }}
        />
        <Stack.Screen 
          name="Register" 
          component={Register}
          options={{ headerShown : false }}
        />
        <Stack.Screen
          name="HomeNavigation"
          component={HomeNavigation}
          options={{ headerShown : false }}
        />
      </Stack.Navigator>
  );
};

export default OnboardingFlow;