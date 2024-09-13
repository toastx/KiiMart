import React from 'react';
import { Box, Heading, Button, Checkbox, VStack } from '@chakra-ui/react';

interface FilterComponentProps {
  onClose: () => void;
}

const FilterComponent: React.FC<FilterComponentProps> = ({ onClose }) => {
  return (
    <Box w="250px" p="4" bg="purple.50" boxShadow="md">
      <VStack align="start" spacing="4">
        <Heading size="md">Filter</Heading>

        <Checkbox>Category 1</Checkbox>
        <Checkbox>Category 2</Checkbox>
        <Checkbox>Category 3</Checkbox>
        <Checkbox>Price Range</Checkbox>

        <Button colorScheme="purple" onClick={onClose} width="100%">
          Apply Filters
        </Button>
      </VStack>
    </Box>
  );
};

export default FilterComponent;
