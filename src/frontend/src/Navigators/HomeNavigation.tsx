import { createBottomTabNavigator } from "@react-navigation/bottom-tabs";
import { Movies } from "../Screens/Movies";
import { Theatres } from "../Screens/Theatres";
import {NavigationContainer} from '@react-navigation/native';
import { SafeAreaProvider } from "react-native-safe-area-context";
import { Profile } from "../Screens/Profile";
import FontAwesome from 'react-native-vector-icons/FontAwesome';


const Tab = createBottomTabNavigator();

export const HomeNavigation = () => {
  return (
    <SafeAreaProvider>
        <Tab.Navigator
          screenOptions={({ route }) => ({
            tabBarIcon: ({ focused, color, size }) => {
              let iconName;

              if (route.name === 'Home') {
                iconName = focused ? 'ios-information-circle' : 'ios-information-circle-outline';
              } else if (route.name === 'Settings') {
                iconName = focused ? 'ios-list' : 'ios-list-outline';
              }

              // You can return any component that you like here!
              return <FontAw name="rocket" size={30} color="#900" />
            },
            tabBarActiveTintColor: 'tomato',
            tabBarInactiveTintColor: 'gray',
        })}
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
