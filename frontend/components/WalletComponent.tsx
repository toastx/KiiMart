import React from 'react';
import { Button, useToast } from '@chakra-ui/react';
import { FC } from "react"
const WalletComponent : FC = () => {

  const toast = useToast();

  const requestWallet = async () => {
    try {
      const accounts = await window.ethereum.request({
        method: "eth_requestAccounts",
      });
      const account = accounts[0];
      toast({
        title: 'Wallet Connected.',
        description: `Wallet connected: ${account}`,
        status: 'success',
        duration: 9000,
        isClosable: true,
      });

    } catch (error) {
      console.log(error);
      
      toast({
        title: 'Error Connecting Wallet.',
        status: 'error',
        duration: 9000,
        isClosable: true,
      });
    }
  };

  return (
    <Button  bgColor={"#f46197ff"}
    fontSize="16"
    onClick={requestWallet}>
      Connect Wallet
    </Button>
  );
};

export default WalletComponent;
