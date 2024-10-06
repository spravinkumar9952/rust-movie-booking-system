



import * as React from 'react';
import {NavigationContainer} from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';
import Register from '../Screens/Register';
import Login from '../Screens/Login';
import { Home } from '../Screens/Home';

const Stack = createStackNavigator();

const OnboardingFlow = () => {
  return (
    <NavigationContainer>
      <Stack.Navigator>
        <Stack.Screen
          name="Login"
          component={Login}
          options={{ headerShown : false }}
        />
        <Stack.Screen 
          name="Register" 
          component={Register} 
        />
        <Stack.Screen
          name="Home"
          component={Home}
        />
      </Stack.Navigator>
    </NavigationContainer>
  );
};

export default OnboardingFlow;