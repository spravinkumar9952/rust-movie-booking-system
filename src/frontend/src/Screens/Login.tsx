import { useState } from "react"
import { Button, Text } from "react-native"
import { Pressable, RawButton, TextInput } from "react-native-gesture-handler"
import { SafeAreaView } from "react-native-safe-area-context"
import { baseUrl } from "../API/Configs"
import { LoginReq, LoginResp, loginUser } from "../API/Login"
import { storeKeyVal } from "../Helpers/LocalStorage"
import { StackNavigationProp } from "@react-navigation/stack"
import { color } from "../Helpers/Colors"


type LoginProps = {
  navigation: StackNavigationProp<any>;
};


const Login : React.FC<LoginProps>= ({navigation}) => {
  const [mobileNumber, setMobileNumber] = useState("")
  const [password, setPassword] = useState("")
  const [showInvalidCredentials, setShowInvalidCredentials] = useState(false)

  const mobileNumberChanged = (text: string) => {
    console.log(text)
    setMobileNumber((oldText) => text)
  }

  const passwordChanged = (text: string) => {
    console.log(text)
    setPassword((oldText) => text)
  }

  const handleSubmit = async () => {
    try{
      let resp: LoginResp = await loginUser({phone_number: mobileNumber, password: password} as LoginReq);
      storeKeyVal("registration_token", resp.registration_token);
      navigation.reset({
        index: 0,
        routes: [{ name: "HomeNavigation" }],
      });
    }catch(e){
      console.log(e);
      setShowInvalidCredentials(true);
    }
  }

  const handleeRegister = () => {
    navigation.navigate("Register")
  }

  return (
    <SafeAreaView style={{backgroundColor: color.primaryBackground, flex: 1, flexDirection: "column", alignItems: "center", justifyContent: "center"}}>
      <Text style={{color : color.primaryText, marginBottom: 16, fontSize: 24, fontWeight: 800}}>Login</Text>

      <TextInput placeholder="Mobile Number" onChangeText={mobileNumberChanged} style={{
          backgroundColor: color.primaryText, 
          width: 200, 
          height: 50, 
          borderRadius: 8
        }} />
      <TextInput placeholder="Password" secureTextEntry = {true} onChangeText={passwordChanged} style={{backgroundColor: color.primaryText, width: 200, height: 50, borderRadius: 8, marginTop: 16}} />

      <Pressable onPress={handleSubmit} style={{backgroundColor: color.secondaryBackground , marginTop: 16, borderRadius:8}}>
        <Text style={{color: color.primaryText, padding: 16}}>Login</Text>
      </Pressable>

      <Pressable onPress={handleeRegister} style={{marginTop: 16, backgroundColor: color.secondaryBackground, borderRadius:8}}>
        <Text style={{color: color.primaryText, padding: 16}}>Register</Text>
      </Pressable>

      <Text style={{color: color.secondaryText, display: showInvalidCredentials ? "flex" : "none", marginTop: 8}}>Invalid Credentials</Text>
    </SafeAreaView>
  )
}

export default Login