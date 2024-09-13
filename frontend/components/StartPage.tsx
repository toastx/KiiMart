import { Button, HStack, Spacer, VStack, Heading, Container,Text } from "@chakra-ui/react"
import { ArrowForwardIcon } from "@chakra-ui/icons";
import { FC } from "react"
import dynamic from "next/dynamic";
import { ethers } from "ethers";


const StartPage: FC = () => {
    return (
        <Container>
          <VStack spacing={8}>
            <Heading
              color="#f46197ff"
              as="h1"
              size="3xl"
              noOfLines={4}
              textAlign="center"
            >
              KiiMart
            </Heading>
            <Heading
              color="#fffdf7ff"
              as="h2"
              size="2xl"
              noOfLines={3}
              textAlign="center"
            >       
              Revolutionized RWA Marketplace
            </Heading>
            
            <HStack spacing={20}>
            <Button
              bgColor="#f46197ff"
              color="#fffdf7ff"
              maxW="380px"
            >
              <HStack>
                <Text>Dive In</Text>
                <ArrowForwardIcon />
              </HStack>
            </Button>
            
            </HStack>
            </VStack>
          
        </Container>
      )
}

export default StartPage