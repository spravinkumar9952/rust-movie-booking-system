/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 */

import React from 'react';
import OnboardingFlow from './Navigators/OnboardingFlow';
import { getVal } from './Helpers/LocalStorage';
import { HomeNavigation } from './Navigators/HomeNavigation';
import { Movies } from './Screens/Movies';
import { Theatres } from './Screens/Theatres';
import {NavigationContainer} from '@react-navigation/native';

const  App: React.FC = () => {
  const [registration_token, setRegistrationToken] = React.useState<string | null>(null);

  React.useEffect(() => {
    const getRegistrationToken = async () => {
      const registration_token = await getVal("registration_token");
      setRegistrationToken(registration_token);
      console.log(registration_token);
    }
    getRegistrationToken();
  }, []);

  return (
  <NavigationContainer>
    {registration_token === null ? <OnboardingFlow /> : <HomeNavigation />}
  </NavigationContainer>
  )
    
}

export default App;
