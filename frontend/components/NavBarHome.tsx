import { Button, HStack, Spacer, Heading,useToast} from "@chakra-ui/react"
import { WarningTwoIcon} from "@chakra-ui/icons";
import { FC } from "react"
import dynamic from "next/dynamic";
import { ethers } from "ethers";
import WalletComponent from "./WalletComponent";


const NavBarHome: FC = () => {
  return (
    <HStack width="full" padding={4}>
        <Spacer />
        <WalletComponent />
    </HStack>
  )
}

export default NavBarHome