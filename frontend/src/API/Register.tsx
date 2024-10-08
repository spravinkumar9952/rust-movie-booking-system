import { baseUrl } from "./Configs"



export type RegisterReq = {
    phone_number: string,
    password: string
}

export type RegisterResp = {
  message : string
}


export const registerUser = async (req : RegisterReq): Promise<RegisterResp> => {
    let url = baseUrl + "/register";
    let response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(req)
    });
    
    if(response.status !== 200){
      let errorResponse: ErrorResponse = await response.json();
      throw errorResponse;
    }

    return await response.json();
}