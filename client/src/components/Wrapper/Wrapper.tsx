import React from 'react';
import { Box, Theme } from '@mui/material';

import makeStyles from '@mui/styles/makeStyles';
import createStyles from '@mui/styles/createStyles';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    wrapper: {
      maxWidth: theme.breakpoints.values.lg - 40,
      margin: '0 1rem',
      [theme.breakpoints.up('lg')]: {
        margin: '0 auto',
      },
    },
  })
);

interface IWrapper {
  children: React.ReactNode;
}

const Wrapper = ({ children }: IWrapper) => {
  const { wrapper } = useStyles();

  return <Box className={wrapper}>{children}</Box>;
};

export default Wrapper;
