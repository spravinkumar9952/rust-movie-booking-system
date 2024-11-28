import { createBottomTabNavigator } from "@react-navigation/bottom-tabs";
import { Movies } from "../Screens/Movies";
import { Theatres } from "../Screens/Theatres";
import {NavigationContainer} from '@react-navigation/native';
import { SafeAreaProvider } from "react-native-safe-area-context";
import { Profile } from "../Screens/Profile";
import OnboardingFlow from "./OnboardingFlow";


const Tab = createBottomTabNavigator();

export const HomeNavigation = () => {
  return (
    <SafeAreaProvider>
        <Tab.Navigator
          screenOptions={
            ({route}) => ({
              // tabBarIcon: ({ focused, color, size }) => {

              // },
              tabBarActiveTintColor: 'tomato',
              tabBarInactiveTintColor: 'gray',
            })
          }
        >
          <Tab.Screen 
            name="Movies" 
            component={Movies} 
          />
          <Tab.Screen name="Theatres" component={Theatres} />
          <Tab.Screen name="Profile" component={Profile} />
        </Tab.Navigator>
    </SafeAreaProvider>
  );
}
