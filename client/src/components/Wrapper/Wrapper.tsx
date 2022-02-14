import React from 'react';
import { Box, styled } from '@mui/material';

const StyledBox = styled(Box)(({ theme }) => ({
  maxWidth: theme.breakpoints.values.lg - 40,
  margin: '0 1rem',
  [theme.breakpoints.up('lg')]: {
    margin: '0 auto',
  },
}));

interface IWrapper {
  children: React.ReactNode;
}

const Wrapper = ({ children }: IWrapper) => (
  <StyledBox>{children}</StyledBox>
);

export default Wrapper;
