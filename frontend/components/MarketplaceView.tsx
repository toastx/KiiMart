import React, { useState } from 'react';
import { Box, Flex, Input, Button, Grid, GridItem, Heading, IconButton } from '@chakra-ui/react';
import { ArrowForwardIcon } from "@chakra-ui/icons";
import FilterComponent from './FilterComponent';

const MarketplaceView: React.FC = () => {
  const [showFilter, setShowFilter] = useState(false);
  const [searchTerm, setSearchTerm] = useState('');

  const toggleFilter = () => setShowFilter(!showFilter);

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchTerm(e.target.value);
  };

  const items = [
    { id: 1, name: 'Item 1' },
    { id: 2, name: 'Item 2' },
    { id: 3, name: 'Item 3' },
    { id: 4, name: 'Item 4' },
    // Add more items
  ];

  const filteredItems = items.filter((item) =>
    item.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <Flex>
      {/* Left sidebar for filter */}
      {showFilter && <FilterComponent onClose={toggleFilter} />}

      {/* Main content area */}
      <Box flex="1" p="4">
        <Flex justify="space-between" mb="4" alignItems="center">
          <Heading>KiiMart Marketplace</Heading>

          {/* Search Input */}
          <Input
            placeholder="Search items..."
            value={searchTerm}
            onChange={handleSearchChange}
            width="300px"
            mx="auto"
          />

          {/* Toggle Filter Button */}
          <IconButton
            aria-label="Toggle Filter"
            icon={<ArrowForwardIcon />}
            onClick={toggleFilter}
          />
        </Flex>

        {/* Marketplace Grid */}
        <Grid templateColumns="repeat(4, 1fr)" gap={6}>
          {filteredItems.map((item) => (
            <GridItem
              key={item.id}
              w="100%"
              h="200px"
              bg="purple.100"
              borderRadius="lg"
              boxShadow="lg"
              p="4"
            >
              {item.name}
            </GridItem>
          ))}
        </Grid>
      </Box>
    </Flex>
  );
};

export default MarketplaceView;
