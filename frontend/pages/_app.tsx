import type { AppProps } from "next/app";
import { ChakraProvider } from "@chakra-ui/react"
import { extendTheme } from "@chakra-ui/react"
import { Web3ReactProvider } from '@web3-react/core'
import { Web3Provider } from "@ethersproject/providers";


const colors = {
  background: "#0a2342ff",
  accent: "#833BBE",
  bodyText: "rgba(255, 255, 255, 0.75)",
}

declare global {
    interface Window {
      ethereum: any;
    }
  }


const theme = extendTheme({ colors })

function MyApp({ Component, pageProps }: AppProps) {
  return (
    
    <ChakraProvider theme={theme}>
		<Component {...pageProps} />
</ChakraProvider>
  )
}

export default MyApp