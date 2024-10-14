import { getVal } from "../Helpers/LocalStorage"
import { baseUrl } from "./Configs"



export type TheatrsListResp = Array<Theatre>

export type Theatre = {
  id: number,
  name: string,
  address: string,
  no_of_screens: number
}

export const getTheatresList = async (limit : number) : Promise<Array<Theatre>> => {
  let url = baseUrl + "/theatre/list?limit=" + limit;
  console.log("URL: ", url);
  let token = await getVal("registration_token");
  console.log("Token: ", token);
  if(token === null){
    throw {message: "Token not found"}
  }
  let resp = await fetch(url, {
      method: "GET",
      headers: {
        "Content-Type": "application",
        "token": token
      }
    }
  );

  console.log("Response: ", resp);

  if(resp.status != 200){
    let errorResponse: ErrorResponse = await resp.json();
    throw errorResponse;
  }

  return await resp.json();
}
