import React, { Children, createContext, useContext, useEffect, useRef, useState } from "react";
import { getVal } from "../Helpers/LocalStorage";



type AuthContextType = {
  registrationToken: string | null;
  setRegistrationToken: (registrationToken: string) => void;
  logOut: () => void
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);



export type AuthProviderProps = {
  children: React.ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({children}) => {
  const [registrationToken, setRegistrationToken] = useState<string | null>(null);


  useEffect(() => {
    const getRegistrationToken = async () => {
      const registrationToken = await getVal("registration_token");
      setRegistrationToken(registrationToken);
    }
    getRegistrationToken();
  }, []);

  const logOut = () => {
    setRegistrationToken(null);
  }

  return (
    <AuthContext.Provider value={{registrationToken, setRegistrationToken, logOut}}>
      {children}
    </AuthContext.Provider>
  )
}

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
