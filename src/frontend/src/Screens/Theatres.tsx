import { SafeAreaView, Text, View } from "react-native"
import { color } from "../Helpers/Colors"
import { useEffect, useState } from "react";
import { getTheatresList, Theatre, TheatrsListResp } from "../API/Theatre";


export const Theatres:React.FC = () => {
  const [theatres, setTheatres] = useState<TheatrsListResp>([]);

  useEffect(() => {

    const fetchTheatres = async () => {
      try {
        const theatres = await getTheatresList(5);
        console.log(theatres);
        setTheatres((old) => theatres);
      } catch (e) {
        console.log(e);
      }
    }
    fetchTheatres();
    return () => {}
  }
  , []);

  return (
    <SafeAreaView>
        {
          theatres.map((theatre: any) => <TheatresCardView theatre={theatre} key={theatre.id} />)
        }
    </SafeAreaView>
  );
}


type TheatresCardViewProps = {
  theatre: Theatre
}


const TheatresCardView : React.FC<TheatresCardViewProps>= ( {theatre}) => {
  return (
    <View >
      {
          <View>
            <Text>{theatre.name}</Text>
          </View>
      }
    </View>
  )
}