import { useEffect, useState } from "react";
import { SafeAreaView, StyleSheet, Text, View } from "react-native";
import { getMovies, Movie, MoviesResp } from "../API/Movies";
import { color } from "../Helpers/Colors";


type HomeProps = { }

export const Home: React.FC<HomeProps> = () => {
  const [movies, setMovies] = useState<MoviesResp>([]);

  useEffect(() => {
    const fetchMovies = async () => {
      const movies = await getMovies({});
      setMovies((old) => movies);
    }
    fetchMovies();

    return () => {
      // This is where you can add any cleanup code
      setMovies((old) => []);
    }
  }, [])

  return (
    <SafeAreaView style={homeStyleSheet.container}>
        {
          movies.map((movie: Movie) => <MoviesCardView movie={movie} />)
        }
    </SafeAreaView>
  );
}

const homeStyleSheet = StyleSheet.create({
  container: {
    backgroundColor: color.primaryBackground,
    flex: 1,
    flexDirection: "column",
    display: "flex",
  }
})


type MoviesCardViewProps = {
  movie: Movie
}

const MoviesCardView : React.FC<MoviesCardViewProps>= ( {movie}) => {
  return (
    <View style={moviesCardViewStyleSheet.container}>
      {
          <View key={movie.id}>
            <Text style={moviesCardViewStyleSheet.title}>{movie.title}</Text>
          </View>
      }
    </View>
  )
}


const moviesCardViewStyleSheet = StyleSheet.create({
  container: {
    backgroundColor: color.secondaryBackground,
    marginTop: 16,
    marginHorizontal: 8,
    padding: 16,
    borderRadius: 8
  },
  title: {
    color: color.primaryText,
    fontSize: 18,
  }
})