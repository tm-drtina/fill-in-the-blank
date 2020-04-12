import { createStyles, makeStyles, Theme } from '@material-ui/core';
import React from 'react';

import Loader from './Loader';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      position: 'fixed',
      top: 0,
      left: 0,
      width: '100%',
      zIndex: 9999,
    },
  })
);

type Props = {
  loaders: number;
};

const LoaderGlobal: React.FC<Props> = ({ loaders }) => {
  const { root } = useStyles();

  return (
    <>
      {loaders > 0 && (
        <div className={root}>
          <Loader />
        </div>
      )}
    </>
  );
};

export default LoaderGlobal;
