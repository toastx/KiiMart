import { Box,Link} from "@chakra-ui/react"
import { FC } from "react"
import dynamic from "next/dynamic";



const Footer: FC = () => {
  return (
    <Box marginBottom={4} color="#fffdf7ff" fontSize="xl" fontWeight={"bold"} >
        Built by
        <Link
        paddingLeft={1}
        href="https://github.com/toastx"
        target="_blank"
        rel="noopener noreferrer"
        color="gold" 
      >
        Toastx
      </Link>
    </Box>
  )
}

export default Footer