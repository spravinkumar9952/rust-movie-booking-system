


type Color = {
  primaryBackground: string,
  secondaryBackground: string,
  primaryText: string,
  secondaryText: string
}

export const lightColors: Color = {
  primaryBackground: "#FF0000",
  secondaryBackground: "#00FF00",
  primaryText: "#FFFFFF",
  secondaryText: "#000000"
}

export const darkColors: Color = {
  primaryBackground: "#FF0000",
  secondaryBackground: "#00FF00",
  primaryText: "#000000",
  secondaryText: "#FFFFFF"
}

export const getColors = (theme: string) => {
    if(theme === "light"){
        return lightColors;
    }else{
        return darkColors;
    }
}

export const color = getColors("light");

