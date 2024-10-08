import { getVal } from "../Helpers/LocalStorage"
import { baseUrl } from "./Configs"




export type MoviesReq = {

}

export type MoviesResp = Array<Movie>

export type Movie = {
  id: string,
  title: string,
  actors: Array<Actor>
}

export type Actor = {
  id: string,
  name: string
}

export const getMovies = async (req : MoviesReq): Promise<MoviesResp> => {
    let url = baseUrl + "/movies";
    let token = await getVal("registration_token");
    if(token === null){
      throw {message: "Token not found"}
    }

    let response = await fetch(url, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "token": token
      }
    });
    
    if(response.status !== 200){
      let errorResponse: ErrorResponse = await response.json();
      throw errorResponse;
    }

    return await response.json();
}