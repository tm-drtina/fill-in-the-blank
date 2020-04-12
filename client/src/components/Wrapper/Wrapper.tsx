import React from 'react';
import { Box, makeStyles, Theme, createStyles } from '@material-ui/core';

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
