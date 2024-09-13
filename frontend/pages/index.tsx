import { Box, Center, Spacer, Stack } from "@chakra-ui/react"
import type { NextPage } from "next"
import Head from "next/head"

import NavBarHome from "@/components/NavBarHome"
import StartPage from "@/components/StartPage"
import Footer from "@/components/Footer"
import styles from '../styles/styles.module.css'
import MarketplaceView from "@/components/MarketPlaceView"
const Home: NextPage = () => {

  return (
    <div className={styles.container}>
      <Head>
        <title>KiiMart</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <Stack w="full" h="calc(100vh)" justify="center">
          <NavBarHome />
          <Spacer />
          <Center>
            <MarketplaceView />
          </Center>
          <Spacer />

          <Center>
            <Footer />
          </Center>
        </Stack>
      
    </div>
  )
}

export default Home