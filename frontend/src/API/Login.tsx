import { baseUrl } from "./Configs"

type LoginReq= {
    phone_number: string,
    password: string
}

type LoginResp = {
    registration_token : string
}

const loginUser = async (req : LoginReq): Promise<LoginResp> => {
    let url = baseUrl + "/login";
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


export { loginUser }
export type { LoginReq, LoginResp }
