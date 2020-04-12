import React from 'react';
import { Typography, Box } from '@material-ui/core';

const Footer = () => {
  return (
    <Box py={4}>
      <Typography align="center">&copy; Tomas Drtina {new Date().getFullYear()}</Typography>
    </Box>
  );
};

export default Footer;
