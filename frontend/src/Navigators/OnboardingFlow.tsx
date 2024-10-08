



import * as React from 'react';
import {NavigationContainer} from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';
import Register from '../Screens/Register';
import Login from '../Screens/Login';
import { Home } from '../Screens/Home';

const Stack = createStackNavigator({
  initialRouteName: "Login"
});

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
          options={{ headerShown : false }}
        />
        <Stack.Screen
          name="Home"
          component={Home}
          options={{ headerShown : false }}
        />
      </Stack.Navigator>
    </NavigationContainer>
  );
};

export default OnboardingFlow;