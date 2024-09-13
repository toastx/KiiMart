import React from 'react';
import { Flex, Heading, Button, useColorModeValue } from '@chakra-ui/react';
import { ColorModeSwitcher } from './ColorModeSwitcher'; // Adjust path if necessary

const Navbar = ({ connectWallet, account }) => {
  const linkColor = useColorModeValue('whiteAlpha.900', 'gray.200');
  const buttonColor = useColorModeValue('purple.500', 'purple.300');

  // Ensure account is a string and format it correctly
  const displayAccount = typeof account === 'string' && account.length > 0
    ? `${account.substring(0, 6)}...${account.substring(account.length - 4)}`
    : '';

  return (
    <Flex as="nav" padding="1.5rem" bg="purple.800" color="white" justifyContent="space-between" alignItems="center">
      <Heading size="md" color={useColorModeValue('white', 'gray.100')}>Marketplace</Heading>
      <Flex gap="1rem" alignItems="center">
        <Button variant="link" color={linkColor} _hover={{ color: buttonColor }}>Home</Button>
        <Button variant="link" color={linkColor} _hover={{ color: buttonColor }}>Inventory</Button>
        <Button variant="link" color={linkColor} _hover={{ color: buttonColor }}>Profile</Button>
        {account ? (
          <Button colorScheme="green" disabled>
            Connected: {displayAccount}
          </Button>
        ) : (
          <Button colorScheme="purple" onClick={connectWallet}>Connect Wallet</Button>
        )}
        <ColorModeSwitcher />
      </Flex>
    </Flex>
  );
};

export default Navbar;
