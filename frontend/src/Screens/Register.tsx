import { StackNavigationProp } from "@react-navigation/stack";
import { Pressable, SafeAreaView, Text, TextInput, View } from "react-native";
import { color } from "../Helpers/Colors";
import { useState } from "react";
import { RegisterReq, RegisterResp, registerUser } from "../API/Register";


type RegisterProps = {
  navigation: StackNavigationProp<any>;
};



const Register : React.FC<RegisterProps>= ({navigation}) => {
  const [mobileNumber, setMobileNumber] = useState("")
  const [password, setPassword] = useState("")
  const [reEnteredPassword, setReEnteredPassword] = useState("") 
  const [errorMsg, setErrorMsg] = useState("")

  const mobileNumberChanged = (text: string) => {
    setMobileNumber((oldText) => text)
  }

  const passwordChanged = (text: string) => {
    setPassword((oldText) => text)
  }

  const ReEnteredPassword = (text: string) => {
    setReEnteredPassword((oldText) => text)
  }

  const goToLogin = () => {
    navigation.pop();
  }

  const handleRegister = async () => {
    if(password != reEnteredPassword){
      setErrorMsg("Passwords do not match")
      return;
    }

    let resp: RegisterResp = await registerUser({phone_number: mobileNumber, password: password} as RegisterReq);
    navigation.navigate("Login");
  }


  return (
    <SafeAreaView style={{backgroundColor: color.primaryBackground, flex: 1, flexDirection: "column", alignItems: "center", justifyContent: "center"}}>
      <Text style={{color : color.primaryText, marginBottom: 16, fontSize: 24, fontWeight: 800}}>Register</Text>

      <TextInput placeholder="Mobile Number" onChangeText={mobileNumberChanged} style={{backgroundColor: color.primaryText, width: 200, height: 50, borderRadius: 8}} />
      <TextInput placeholder="Password" secureTextEntry = {true} onChangeText={passwordChanged} style={{backgroundColor: color.primaryText, width: 200, height: 50, borderRadius: 8, marginTop: 16}} />
      <TextInput placeholder="ReEnteredPassword" secureTextEntry = {true} onChangeText={ReEnteredPassword} style={{backgroundColor: color.primaryText, width: 200, height: 50, borderRadius: 8, marginTop: 16}} />

      <Pressable onPress={handleRegister} style={{backgroundColor: color.secondaryBackground , marginTop: 16, borderRadius:8}}>
        <Text style={{color: color.primaryText, padding: 16}}>Login</Text>
      </Pressable>

      <Pressable onPress={goToLogin} style={{marginTop: 16, backgroundColor: color.secondaryBackground, borderRadius:8}}>
        <Text style={{color: color.primaryText, padding: 16}}>Go To Login</Text>
      </Pressable>

      <Text style={{color: color.secondaryText, marginTop: 8,  display: errorMsg != ""  ? "flex" : "none"}}>errorMsg</Text>
    </SafeAreaView>
  );
}

export default Register;