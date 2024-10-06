import { SafeAreaView, Text, View } from "react-native";


type HomeProps = { }

export const Home: React.FC<HomeProps> = () => {
  return (
    <SafeAreaView>
      <View style={{backgroundColor: "#000000"}}>
        <Text style={{color: "#FFFFFF"}}>Home</Text>
      </View>
    </SafeAreaView>
  );
}